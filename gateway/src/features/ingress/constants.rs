// Constants for ingress module
// This file centralizes all hardcoded values and constants
// In production, these will be loaded from configuration files or environment variables

/// AI response role constants
pub const AI_RESPONSE_ROLE: &str = "assistant";

/// AI response finish reasons
pub const FINISH_REASON_STOP: &str = "stop";
pub const FINISH_REASON_LENGTH: &str = "length";
pub const FINISH_REASON_CONTENT_FILTER: &str = "content_filter";

/// Rewrite prompt prefix
pub const REWRITE_PREFIX: &str = "[Rewritten]";

/// Default model names
pub const DEFAULT_MODEL: &str = "gpt-4";
pub const PREMIUM_MODEL: &str = "gpt-4-turbo";

/// Cost constants (in USD)
pub const DEFAULT_COST: f64 = 0.002;
pub const CACHE_MISS_COST: f64 = 0.005;
pub const PREMIUM_COST: f64 = 0.010;

/// Service timeout constants (in milliseconds)
pub const MEMORY_SERVICE_TIMEOUT_MS: u64 = 5000;
pub const ROUTER_SERVICE_TIMEOUT_MS: u64 = 10000;
pub const REWRITE_SERVICE_TIMEOUT_MS: u64 = 3000;

/// Error messages
pub const ERROR_EMPTY_PROMPT: &str = "Prompt cannot be empty";
pub const ERROR_CONTEXT_RETRIEVAL: &str = "Failed to retrieve conversation context";
pub const ERROR_ROUTING_FAILED: &str = "Failed to route request to AI service";
pub const ERROR_CONTEXT_UPDATE: &str = "Failed to update conversation context";

/// Validation constants
pub const MIN_PROMPT_LENGTH: usize = 1;
pub const MAX_PROMPT_LENGTH: usize = 4000;
pub const MAX_METADATA_SIZE: usize = 1000;
