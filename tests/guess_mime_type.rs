#[cfg(test)]
mod tests {
    use mimesorter::guess_mime_type;
    use std::path::Path;

    #[test]
    fn test_guess_mime_type_html() {
        let path = Path::new("tests/test_files/basic.html");
        let expected_mime_type = "text_html";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }

    #[test]
    fn test_guess_mime_type_xml() {
        let path = Path::new("tests/test_files/note.xml");
        let expected_mime_type = "text_xml";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }

    #[test]
    fn test_guess_mime_type_plain() {
        let path = Path::new("tests/test_files/plaintext.txt");
        let expected_mime_type = "text_plain";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }

    #[test]
    fn test_guess_mime_type_docx() {
        let path = Path::new("tests/test_files/demo.docx");
        let expected_mime_type = "application_vnd.openxmlformats-officedocument.wordprocessingml.document";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }

    #[test]
    fn test_guess_mime_type_pdf() {
        let path = Path::new("tests/test_files/git-cheat-sheet-education.pdf");
        let expected_mime_type = "application_pdf";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }

    #[test]
    fn test_guess_mime_type_png() {
        let path = Path::new("tests/test_files/octocat.png");
        let expected_mime_type = "image_png";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }

    #[test]
    fn test_guess_mime_type_jpg() {
        let path = Path::new("tests/test_files/jpeg420exif.jpg");
        let expected_mime_type = "image_jpeg";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }

    #[test]
    fn test_guess_mime_type_ogg() {
        let path = Path::new("tests/test_files/Clock_ticking.ogg");
        let expected_mime_type = "video_ogg";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }

    #[test]
    fn test_guess_mime_type_wav() {
        let path = Path::new("tests/test_files/amen-break.wav");
        let expected_mime_type = "audio_x-wav";
        let mime_type = guess_mime_type(path).unwrap();
        assert_eq!(mime_type, expected_mime_type);
    }
}
