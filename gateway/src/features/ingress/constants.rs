// Constants for ingress module
// This file centralizes all hardcoded values and constants
// In production, these will be loaded from configuration files or environment variables

/// AI response role for assistant messages.
pub const AI_RESPONSE_ROLE: &str = "assistant";

/// AI response completion reasons.
pub const FINISH_REASON_STOP: &str = "stop";
pub const FINISH_REASON_LENGTH: &str = "length";
pub const FINISH_REASON_CONTENT_FILTER: &str = "content_filter";

/// Prefix added to rewritten prompts.
pub const REWRITE_PREFIX: &str = "[Rewritten]";

/// Default AI model names.
pub const DEFAULT_MODEL: &str = "gpt-4";
pub const PREMIUM_MODEL: &str = "gpt-4-turbo";

/// Request cost constants (in USD).
pub const DEFAULT_COST: f64 = 0.002;
pub const CACHE_MISS_COST: f64 = 0.005;
pub const PREMIUM_COST: f64 = 0.010;

/// Service timeout constants (in milliseconds).
pub const MEMORY_SERVICE_TIMEOUT_MS: u64 = 5000;
pub const ROUTER_SERVICE_TIMEOUT_MS: u64 = 10000;
pub const REWRITE_SERVICE_TIMEOUT_MS: u64 = 3000;

pub const CONTEXT_CACHE_TTL_SECS: u64 = 10;
pub const CONTEXT_CACHE_MAX_ENTRIES: usize = 1000;
pub const SLOW_REQUEST_THRESHOLD_MS: u64 = 1000;

// Error messages are now handled by IngressError enum in error.rs

/// Request validation limits.
pub const MIN_PROMPT_LENGTH: usize = 1;
pub const MAX_PROMPT_LENGTH: usize = 4000;
pub const MAX_METADATA_SIZE: usize = 1000;
