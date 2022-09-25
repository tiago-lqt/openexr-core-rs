use openexr_core::ContextFlags;

#[test]
fn test_context_flags_with_none() {
    {
        let flags = ContextFlags::default().with_strict_header();

        assert!(flags.has_strict_header());
        assert!(!flags.has_silent_header_parse());
        assert!(!flags.has_disable_chunk_reconstruction());
    }
    {
        let flags = ContextFlags::default().with_silent_header_parse();

        assert!(!flags.has_strict_header());
        assert!(flags.has_silent_header_parse());
        assert!(!flags.has_disable_chunk_reconstruction());
    }
    {
        let flags = ContextFlags::default().with_disable_chunk_reconstruction();

        assert!(!flags.has_strict_header());
        assert!(!flags.has_silent_header_parse());
        assert!(flags.has_disable_chunk_reconstruction());
    }
}

#[test]
fn test_context_flags_with_all() {
    {
        let flags = ContextFlags::all().with_strict_header();

        assert!(flags.has_strict_header());
        assert!(flags.has_silent_header_parse());
        assert!(flags.has_disable_chunk_reconstruction());
    }
    {
        let flags = ContextFlags::all().with_silent_header_parse();

        assert!(flags.has_strict_header());
        assert!(flags.has_silent_header_parse());
        assert!(flags.has_disable_chunk_reconstruction());
    }
    {
        let flags = ContextFlags::all().with_disable_chunk_reconstruction();

        assert!(flags.has_strict_header());
        assert!(flags.has_silent_header_parse());
        assert!(flags.has_disable_chunk_reconstruction());
    }
}

#[test]
fn test_context_flags_without_none() {
    {
        let flags = ContextFlags::default().without_strict_header();

        assert!(!flags.has_strict_header());
        assert!(!flags.has_silent_header_parse());
        assert!(!flags.has_disable_chunk_reconstruction());
    }
    {
        let flags = ContextFlags::default().without_silent_header_parse();

        assert!(!flags.has_strict_header());
        assert!(!flags.has_silent_header_parse());
        assert!(!flags.has_disable_chunk_reconstruction());
    }
    {
        let flags = ContextFlags::default().without_disable_chunk_reconstruction();

        assert!(!flags.has_strict_header());
        assert!(!flags.has_silent_header_parse());
        assert!(!flags.has_disable_chunk_reconstruction());
    }
}

#[test]
fn test_context_flags_without_all() {
    {
        let flags = ContextFlags::all().without_strict_header();

        assert!(!flags.has_strict_header());
        assert!(flags.has_silent_header_parse());
        assert!(flags.has_disable_chunk_reconstruction());
    }
    {
        let flags = ContextFlags::all().without_silent_header_parse();

        assert!(flags.has_strict_header());
        assert!(!flags.has_silent_header_parse());
        assert!(flags.has_disable_chunk_reconstruction());
    }
    {
        let flags = ContextFlags::all().without_disable_chunk_reconstruction();

        assert!(flags.has_strict_header());
        assert!(flags.has_silent_header_parse());
        assert!(!flags.has_disable_chunk_reconstruction());
    }
}
