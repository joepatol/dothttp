use std::collections::HashMap;

use winnow::Result as WResult;
use winnow::Parser;
use winnow::token::{literal, take_until};

use crate::error::ParseError;
use crate::models::Request;

/// Line-number metadata for one parsed request, threaded from the parser to
/// the interpolation pass so that `UndefinedVariable` errors report the
/// correct source line.
pub(crate) struct RequestMeta {
    pub(crate) url_line: usize,
    pub(crate) header_lines: Vec<usize>,
    pub(crate) body_line: usize,
}

impl Request {
    /// Substitute every `{{name}}` reference in the request's URL, header
    /// values, and body with the matching variable value.
    ///
    /// Returns `ParseError::UndefinedVariable` if a reference has no
    /// corresponding entry in `vars`.
    pub(crate) fn interpolate(
        self,
        vars: &HashMap<&str, &str>,
        meta: &RequestMeta,
    ) -> Result<Self, ParseError> {
        let url = interpolate_str(&self.url, vars, meta.url_line)?;

        let headers = self
            .headers
            .into_iter()
            .zip(&meta.header_lines)
            .map(|((key, val), &ln)| Ok((key, interpolate_str(&val, vars, ln)?)))
            .collect::<Result<Vec<_>, ParseError>>()?;

        let body = match self.body {
            Some(b) => Some(interpolate_str(&b, vars, meta.body_line)?),
            None => None,
        };

        Ok(Request { url, headers, body, ..self })
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Replace every `{{name}}` reference in `value` with the matching entry from
/// `vars`, returning `ParseError::UndefinedVariable` for any missing key.
fn interpolate_str(
    value: &str,
    vars: &HashMap<&str, &str>,
    line: usize,
) -> Result<String, ParseError> {
    let mut result = String::with_capacity(value.len());
    let mut input = value;

    loop {
        match winnow_scan_ref(&mut input) {
            Ok((literal_part, ref_name)) => {
                result.push_str(&literal_part);
                match vars.get(ref_name) {
                    Some(val) => result.push_str(val),
                    None => {
                        return Err(ParseError::UndefinedVariable {
                            name: ref_name.to_string(),
                            line,
                        })
                    }
                }
            }
            Err(_) => {
                // No more `{{` references — push the remaining input as a literal.
                result.push_str(input);
                break;
            }
        }
    }

    Ok(result)
}

/// Consume everything up to the next `{{name}}` reference and return
/// `(text_before, variable_name)`.  Fails (backtracks) if no `{{` is found.
fn winnow_scan_ref<'s>(input: &mut &'s str) -> WResult<(String, &'s str)> {
    let literal_part = take_until(0.., "{{").parse_next(input)?;
    literal("{{").parse_next(input)?;
    let name = take_until(1.., "}}").parse_next(input)?;
    literal("}}").parse_next(input)?;
    Ok((literal_part.to_string(), name.trim()))
}
