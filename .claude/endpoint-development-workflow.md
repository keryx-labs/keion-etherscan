# Keion Etherscan Endpoint Development Workflow

This guide provides a systematic approach for implementing Etherscan API endpoints following established patterns and best practices.

## 🎯 Objective

Implement all remaining Etherscan API endpoints by following GitHub issues in order, creating comprehensive implementations with full test coverage.

## 📋 Issue Processing Workflow

### Step 1: Issue Analysis & Branch Creation

1. **Start from current feature branch** (progressive development)

   ```bash
   # For the first issue after accounts (Issue #2):
   git checkout develop && git pull origin develop

   # For subsequent issues (Issue #3+), start from previous feature branch:
   git checkout feat/[PREVIOUS_ENDPOINT]  # e.g., feat/contracts for Issue #3
   ```

2. **Identify next issue**

   - Go to: https://github.com/keryx-labs/keion-etherscan/issues
   - Process issues in numerical order
   - Read issue description thoroughly

3. **Create progressive feature branch**
   ```bash
   git checkout -b feat/[ENDPOINT_NAME]
   ```
   Examples (progressive branching):
   - `feat/contracts` - for contract-related endpoints (from develop)
   - `feat/blocks` - for block-related endpoints (from feat/contracts)
   - `feat/transactions` - for transaction endpoints (from feat/blocks)
   - `feat/tokens` - for token endpoints (from feat/transactions)
   - `feat/stats` - for statistics endpoints (from feat/tokens)

### Step 2: Implementation Phase

#### 2.1 Endpoint Module Development

1. **Create/Update endpoint module** in `endpoints/[name].rs`

   - Follow the pattern established in `endpoints/accounts.rs`
   - Use consistent naming conventions
   - Implement builder pattern for complex queries

2. **Key implementation patterns to follow:**

   ```rust
   // Endpoint struct
   pub struct [EndpointName]<'a> {
       client: &'a EtherscanClient,
   }

   // Implementation with builder methods
   impl<'a> [EndpointName]<'a> {
       pub fn new(client: &'a EtherscanClient) -> Self {
           Self { client }
       }

       // Main endpoint methods
       pub async fn [method_name](&self, params) -> Result<[ReturnType]> {
           // Implementation
       }

       // Builder methods for complex queries
       pub fn [query_name](&self, params) -> [QueryBuilder] {
           [QueryBuilder]::new(self.client, params)
       }
   }

   // Query builders with pagination and filtering
   pub struct [QueryBuilder]<'a> {
       client: &'a EtherscanClient,
       // Query parameters
       pagination: Pagination,
   }
   ```

3. **Add getter methods for testing**

   ```rust
   // Getter methods for testing (always add these)
   pub fn get_[field_name](&self) -> [ReturnType] {
       &self.[field_name]
   }

   pub fn get_pagination(&self) -> &Pagination {
       &self.pagination
   }
   ```

#### 2.2 Model Development

1. **Create model structs** in `models/[name].rs`

   - Use `serde` for JSON deserialization
   - Follow existing patterns in `models/` directory
   - Add helper methods for common operations

2. **Model patterns:**

   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct [ModelName] {
       // Fields matching Etherscan API response
   }

   impl [ModelName] {
       // Helper methods for common operations
       pub fn [helper_method](&self) -> [ReturnType] {
           // Implementation
       }
   }
   ```

#### 2.3 Update Module Exports

1. **Update `endpoints/mod.rs`**

   ```rust
   pub mod [endpoint_name];
   ```

2. **Update main `lib.rs`**

   ```rust
   // Add to EtherscanClient impl
   pub fn [endpoint_name](&self) -> endpoints::[EndpointName] {
       endpoints::[EndpointName]::new(self)
   }
   ```

3. **Update `models/mod.rs`**
   ```rust
   pub mod [model_name];
   pub use [model_name]::*;
   ```

### Step 3: Test Development

#### 3.1 Unit Tests

1. **Create test file** `tests/test_[endpoint]_endpoints.rs`

   - Follow pattern from `tests/test_accounts_endpoints.rs`
   - Test all builder methods and parameter validation
   - Test query construction and parameter serialization

2. **Test categories to include:**

   ```rust
   mod builder_tests {
       // Test query builder construction and parameters
   }

   mod parameter_validation_tests {
       // Test input validation and error handling
   }

   mod edge_case_tests {
       // Test boundary conditions and edge cases
   }

   mod network_tests {
       // Test network-specific functionality
   }
   ```

#### 3.2 Integration Tests

1. **Add integration tests** to `tests/test_integration.rs`
   - Add workflow tests for the new endpoint
   - Test interaction with other endpoints
   - Test error handling scenarios

#### 3.3 Model Tests

1. **Add model tests** to `tests/test_models.rs`
   - Test deserialization with sample data
   - Test helper methods
   - Test edge cases and error conditions

### Step 4: Verification & Quality Assurance

#### 4.1 Run Test Suite

```bash
# Run all tests to ensure nothing is broken
cargo test

# Run specific endpoint tests
cargo test --test test_[endpoint]_endpoints

# Run integration tests
cargo test --test test_integration

# Run doc tests
cargo test --doc
```

#### 4.2 Code Quality Checks

```bash
# Check for compilation warnings
cargo clippy

# Format code
cargo fmt

# Check for unused dependencies
cargo machete
```

#### 4.3 Validation Checklist

- [ ] All new tests pass
- [ ] Existing tests still pass
- [ ] No compilation errors or warnings
- [ ] Documentation examples compile
- [ ] Getter methods added for all query builders
- [ ] Error handling properly implemented
- [ ] API follows established patterns

### Step 5: Git Workflow

#### 5.1 Commit Changes

```bash
git add .
git commit -m "[Comprehensive commit message without any Claude attribution]"
```

**Commit message template:**

```
Implement [ENDPOINT_NAME] endpoints with comprehensive testing

This commit adds full support for [ENDPOINT_DESCRIPTION] through the Etherscan API.

## Key Features
- [Feature 1]: [Description]
- [Feature 2]: [Description]
- [Feature 3]: [Description]

## API Endpoints Added
- [method_name()]: [Description]
- [method_name()]: [Description]

## Query Builders
- [QueryBuilder]: [Description with capabilities]

## Test Coverage
- [X] Unit tests: [N] tests covering all functionality
- [X] Integration tests: [N] workflow tests
- [X] Error handling: [N] error scenarios tested
- [X] Edge cases: [N] boundary condition tests

## Model Support
- [ModelName]: [Description]
- [ModelName]: [Description]

Closes #[ISSUE_NUMBER]
```

#### 5.2 Push and Create MR

```bash
# Push branch
git push -u origin feat/[endpoint]

# Create merge request
gh pr create --base develop --title "[DESCRIPTIVE_TITLE]" --body "[COMPREHENSIVE_MR_DESCRIPTION without any mention of being coauthored by Claude!]"
```

**MR Description Template:**

```markdown
## Summary

[Brief description of what this MR implements]

## Problem Statement

[Description of the GitHub issue being solved]

## Solution Overview

[High-level description of the implementation approach]

### 🔧 Endpoints Implemented

- `[method_name]()` - [Description]
- `[method_name]()` - [Description]

### 🏗️ Query Builders Added

- `[QueryBuilder]` - [Capabilities and features]

### 📊 Models Added

- `[ModelName]` - [Purpose and usage]

## Test Results
```

✅ [Endpoint] Tests: [N]/[N] passing
✅ Integration Tests: [N]/[N] passing  
✅ Model Tests: [N]/[N] passing
✅ Total: [N] tests passing, 0 failing

```

## Key Benefits
1. [Benefit 1]
2. [Benefit 2]
3. [Benefit 3]

## Breaking Changes
[None/Description of any breaking changes]

Closes https://github.com/keryx-labs/keion-etherscan/issues/[N]
```

### Step 6: Iterate to Next Issue

1. **Merge current MR** (after review/approval)
2. **Continue progressively from current branch**
   ```bash
   # Stay on current feature branch to build next feature on top
   # Next issue will branch from this feature branch
   ```
3. **Repeat from Step 1** with next issue (branching from current feature)

## 🔍 Implementation Guidelines

### Code Quality Standards

1. **Consistency**: Follow patterns established in `endpoints/accounts.rs`
2. **Testing**: Achieve >95% test coverage for new code
3. **Documentation**: Include comprehensive doc comments and examples
4. **Error Handling**: Proper error propagation and user-friendly messages
5. **Performance**: Efficient parameter serialization and response parsing

### Common Patterns to Follow

1. **Builder Pattern**: Use for complex queries with multiple optional parameters
2. **Pagination**: Implement consistent pagination across all endpoints
3. **Validation**: Input validation before API calls
4. **Normalization**: Address and hash normalization where applicable
5. **Type Safety**: Strong typing for all API parameters and responses

### Issues Processing Order

Process GitHub issues in numerical order:

1. ✅ Issue #2 - Contracts endpoints (COMPLETED)
2. Issue #3 - Blocks endpoints
3. Issue #4 - Transactions endpoints
4. Issue #5 - Tokens endpoints
5. Issue #6 - Statistics endpoints
6. [Continue with subsequent issues...]

## ⚠️ Important Notes

- **Use progressive branching** - Each new issue branches from the previous feature branch, not from develop
- **Never mention Claude authorship** in commits or MRs (e.g., no "Co-Authored-By: Claude" or "Generated with Claude Code")
- **Always run full test suite** before committing
- **Follow semantic commit messages** for clear git history
- **Link all MRs to their corresponding issues**
- **Maintain backward compatibility** unless explicitly breaking changes are needed
- **Add getter methods** to all query builders for testing support

This workflow ensures consistent, high-quality implementations across all endpoints while maintaining the project's established patterns and standards.

After all of the issues is done, open an MR from develop into main.
