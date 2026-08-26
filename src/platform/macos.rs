use super::AccessRequest;
use crate::types::{KeyboardLayout, SystemCheck, SystemCheckItem};
use objc2_core_graphics::{
    CGEvent, CGEventFlags, CGEventSource, CGEventSourceStateID, CGEventTapLocation,
    CGPreflightPostEventAccess, CGRequestPostEventAccess,
};
use std::process::Command;

pub(super) fn check_system(access_request: AccessRequest) -> SystemCheck {
    let posting_access = posting_access(
        access_request,
        || CGPreflightPostEventAccess(),
        || CGRequestPostEventAccess(),
    );
    let tesseract = Command::new("tesseract")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success());

    build_system_check(posting_access, tesseract)
}

pub(super) const fn recheck_readiness_on_activation() -> bool {
    true
}

pub(super) fn prepare_typing() -> Result<(), String> {
    prepare_typing_with(CGPreflightPostEventAccess())
}

pub(super) fn type_character(ch: char, layout: KeyboardLayout) -> Result<(), String> {
    let sequence = match layout {
        KeyboardLayout::Us => us_sequence(ch).ok_or_else(|| {
            format!("character {ch:?} has no physical key mapping for the macOS US keyboard layout")
        })?,
        KeyboardLayout::Norwegian => {
            return Err(
                "the macOS Norwegian keyboard layout is not supported by this typing backend yet"
                    .to_string(),
            );
        }
    };
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).ok_or_else(|| {
        "Core Graphics could not create a keyboard event source; verify Accessibility permission in System Settings > Privacy & Security"
            .to_string()
    })?;

    post_sequence_with(&sequence, |transition, shift_held| {
        let event =
            CGEvent::new_keyboard_event(Some(&source), transition.key_code, transition.key_down)
                .ok_or_else(|| {
                    format!(
                        "Core Graphics could not create keyboard event for key code {}",
                        transition.key_code
                    )
                })?;

        let flags = if shift_held {
            CGEventFlags::MaskShift
        } else {
            CGEventFlags::empty()
        };
        CGEvent::set_flags(Some(&event), flags);
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(&event));
        Ok(())
    })
}

fn prepare_typing_with(posting_access: bool) -> Result<(), String> {
    if posting_access {
        Ok(())
    } else {
        Err("Keyboard event posting access is required. Allow Jumpbox Typer in System Settings > Privacy & Security > Accessibility, then run Check system again.".to_string())
    }
}

fn posting_access(
    access_request: AccessRequest,
    mut preflight: impl FnMut() -> bool,
    mut request: impl FnMut() -> bool,
) -> bool {
    let allowed = preflight();
    if !allowed && access_request == AccessRequest::RequestIfNeeded {
        request()
    } else {
        allowed
    }
}

fn build_system_check(posting_access: bool, tesseract: bool) -> SystemCheck {
    SystemCheck {
        items: vec![
            SystemCheckItem {
                title: "Keyboard event posting access".to_string(),
                ok: posting_access,
                detail: if posting_access {
                    "Jumpbox Typer can post keyboard events.".to_string()
                } else {
                    "Allow Jumpbox Typer in System Settings > Privacy & Security > Accessibility, then return to the app.".to_string()
                },
                help: "macOS requires Accessibility permission before Jumpbox Typer can send physical keyboard events to the focused application.".to_string(),
            },
            SystemCheckItem {
                title: "tesseract OCR installed".to_string(),
                ok: tesseract,
                detail: if tesseract {
                    "tesseract OCR is installed.".to_string()
                } else {
                    "Install tesseract OCR with Homebrew: brew install tesseract".to_string()
                },
                help: "Tesseract reads text from clipboard images and screenshots. The OCR button depends on it to turn a screenshot into editable text.".to_string(),
            },
        ],
        can_type: posting_access,
        can_ocr: tesseract,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct KeyTransition {
    key_code: u16,
    key_down: bool,
}

const SHIFT_KEY_CODE: u16 = 56;

fn us_sequence(ch: char) -> Option<Vec<KeyTransition>> {
    let (key_code, shifted) = match ch {
        '\n' => (36, false),
        '\t' => (48, false),
        ' ' => (49, false),
        '1' => (18, false),
        '2' => (19, false),
        '3' => (20, false),
        '4' => (21, false),
        '5' => (23, false),
        '6' => (22, false),
        '7' => (26, false),
        '8' => (28, false),
        '9' => (25, false),
        '0' => (29, false),
        '!' => (18, true),
        '"' => (39, true),
        '#' => (20, true),
        '$' => (21, true),
        '%' => (23, true),
        '&' => (26, true),
        '\'' => (39, false),
        '(' => (25, true),
        ')' => (29, true),
        '*' => (28, true),
        '+' => (24, true),
        ',' => (43, false),
        '-' => (27, false),
        '.' => (47, false),
        '/' => (44, false),
        ':' => (41, true),
        ';' => (41, false),
        '<' => (43, true),
        '=' => (24, false),
        '>' => (47, true),
        '?' => (44, true),
        '@' => (19, true),
        '[' => (33, false),
        '\\' => (42, false),
        ']' => (30, false),
        '^' => (22, true),
        '_' => (27, true),
        '`' => (50, false),
        '{' => (33, true),
        '|' => (42, true),
        '}' => (30, true),
        '~' => (50, true),
        _ if ch.is_ascii_alphabetic() => {
            let key_code = match ch.to_ascii_lowercase() {
                'a' => 0,
                'b' => 11,
                'c' => 8,
                'd' => 2,
                'e' => 14,
                'f' => 3,
                'g' => 5,
                'h' => 4,
                'i' => 34,
                'j' => 38,
                'k' => 40,
                'l' => 37,
                'm' => 46,
                'n' => 45,
                'o' => 31,
                'p' => 35,
                'q' => 12,
                'r' => 15,
                's' => 1,
                't' => 17,
                'u' => 32,
                'v' => 9,
                'w' => 13,
                'x' => 7,
                'y' => 16,
                'z' => 6,
                _ => unreachable!(),
            };
            (key_code, ch.is_ascii_uppercase())
        }
        _ => return None,
    };

    Some(key_sequence(key_code, shifted))
}

fn key_sequence(key_code: u16, shifted: bool) -> Vec<KeyTransition> {
    let mut sequence = Vec::new();
    if shifted {
        sequence.push(KeyTransition {
            key_code: SHIFT_KEY_CODE,
            key_down: true,
        });
    }
    sequence.extend([
        KeyTransition {
            key_code,
            key_down: true,
        },
        KeyTransition {
            key_code,
            key_down: false,
        },
    ]);
    if shifted {
        sequence.push(KeyTransition {
            key_code: SHIFT_KEY_CODE,
            key_down: false,
        });
    }
    sequence
}

fn post_sequence_with(
    sequence: &[KeyTransition],
    mut post: impl FnMut(KeyTransition, bool) -> Result<(), String>,
) -> Result<(), String> {
    let mut shift_held = false;
    let mut first_error = None;

    for &transition in sequence {
        if transition.key_code == SHIFT_KEY_CODE {
            shift_held = transition.key_down;
        }

        if let Err(error) = post(transition, shift_held) {
            if first_error.is_none() {
                first_error = Some(error);
            }
        }
    }

    first_error.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn missing_posting_access_reports_macos_readiness_without_linux_concepts() {
        let check = build_system_check(false, true);

        assert!(!check.can_type);
        assert!(check.can_ocr);
        assert_eq!(
            check
                .items
                .iter()
                .map(|item| item.title.as_str())
                .collect::<Vec<_>>(),
            vec!["Keyboard event posting access", "tesseract OCR installed"]
        );
        assert!(check.items[0].detail.contains("Privacy & Security"));
        assert!(check.items[0].detail.contains("Accessibility"));
        assert!(!check.items.iter().any(|item| {
            item.title.contains("ydotool")
                || item.detail.contains("ydotool")
                || item.detail.contains("uinput")
                || item.detail.contains("systemctl")
        }));
    }

    #[test]
    fn startup_check_preflights_without_requesting_access() {
        let request_count = Cell::new(0);

        let allowed = posting_access(
            AccessRequest::CheckOnly,
            || false,
            || {
                request_count.set(request_count.get() + 1);
                false
            },
        );

        assert!(!allowed);
        assert_eq!(request_count.get(), 0);
    }

    #[test]
    fn user_check_requests_missing_access_once() {
        let request_count = Cell::new(0);

        let allowed = posting_access(
            AccessRequest::RequestIfNeeded,
            || false,
            || {
                request_count.set(request_count.get() + 1);
                true
            },
        );

        assert!(allowed);
        assert_eq!(request_count.get(), 1);
    }

    #[test]
    fn us_letters_map_to_physical_keys_with_shift_for_uppercase() {
        let key_codes = [
            ('a', 0),
            ('b', 11),
            ('c', 8),
            ('d', 2),
            ('e', 14),
            ('f', 3),
            ('g', 5),
            ('h', 4),
            ('i', 34),
            ('j', 38),
            ('k', 40),
            ('l', 37),
            ('m', 46),
            ('n', 45),
            ('o', 31),
            ('p', 35),
            ('q', 12),
            ('r', 15),
            ('s', 1),
            ('t', 17),
            ('u', 32),
            ('v', 9),
            ('w', 13),
            ('x', 7),
            ('y', 16),
            ('z', 6),
        ];

        for (lowercase, key_code) in key_codes {
            assert_eq!(us_sequence(lowercase), Some(key_sequence(key_code, false)));
            assert_eq!(
                us_sequence(lowercase.to_ascii_uppercase()),
                Some(key_sequence(key_code, true))
            );
        }
    }

    #[test]
    fn us_digits_and_whitespace_controls_map_to_physical_keys() {
        let mappings = [
            ('1', 18),
            ('2', 19),
            ('3', 20),
            ('4', 21),
            ('5', 23),
            ('6', 22),
            ('7', 26),
            ('8', 28),
            ('9', 25),
            ('0', 29),
            ('\n', 36),
            ('\t', 48),
            (' ', 49),
        ];

        for (character, key_code) in mappings {
            assert_eq!(
                us_sequence(character),
                Some(key_sequence(key_code, false)),
                "wrong mapping for {character:?}"
            );
        }
    }

    #[test]
    fn us_printable_punctuation_maps_to_physical_keys() {
        let mappings = [
            ('!', 18, true),
            ('"', 39, true),
            ('#', 20, true),
            ('$', 21, true),
            ('%', 23, true),
            ('&', 26, true),
            ('\'', 39, false),
            ('(', 25, true),
            (')', 29, true),
            ('*', 28, true),
            ('+', 24, true),
            (',', 43, false),
            ('-', 27, false),
            ('.', 47, false),
            ('/', 44, false),
            (':', 41, true),
            (';', 41, false),
            ('<', 43, true),
            ('=', 24, false),
            ('>', 47, true),
            ('?', 44, true),
            ('@', 19, true),
            ('[', 33, false),
            ('\\', 42, false),
            (']', 30, false),
            ('^', 22, true),
            ('_', 27, true),
            ('`', 50, false),
            ('{', 33, true),
            ('|', 42, true),
            ('}', 30, true),
            ('~', 50, true),
        ];

        for (character, key_code, shifted) in mappings {
            assert_eq!(
                us_sequence(character),
                Some(key_sequence(key_code, shifted)),
                "wrong mapping for {character:?}"
            );
        }
    }

    #[test]
    fn every_guaranteed_us_sequence_releases_shift() {
        let corpus = (b' '..=b'~').map(char::from).chain(['\n', '\t']);

        for character in corpus {
            let sequence = us_sequence(character).expect("guaranteed character must map");
            let mut shift_depth = 0_i32;

            for transition in sequence {
                if transition.key_code == SHIFT_KEY_CODE {
                    shift_depth += if transition.key_down { 1 } else { -1 };
                    assert!(
                        shift_depth >= 0,
                        "Shift released before press for {character:?}"
                    );
                }
            }

            assert_eq!(shift_depth, 0, "Shift left held for {character:?}");
        }
    }

    #[test]
    fn posting_failure_still_attempts_modifier_release() {
        let sequence = key_sequence(0, true);
        let mut attempted = Vec::new();

        let error = post_sequence_with(&sequence, |transition, shift_held| {
            attempted.push((transition, shift_held));
            if transition.key_code == 0 && transition.key_down {
                Err("could not create keyboard event".to_string())
            } else {
                Ok(())
            }
        })
        .unwrap_err();

        assert_eq!(error, "could not create keyboard event");
        assert_eq!(
            attempted,
            vec![
                (sequence[0], true),
                (sequence[1], true),
                (sequence[2], true),
                (sequence[3], false),
            ]
        );
    }

    #[test]
    fn typing_preparation_rejects_revoked_posting_access() {
        let error = prepare_typing_with(false).unwrap_err();

        assert!(error.contains("Privacy & Security"));
        assert!(error.contains("Accessibility"));
    }

    #[test]
    fn macos_rechecks_readiness_when_application_returns_to_foreground() {
        assert!(recheck_readiness_on_activation());
    }
}
