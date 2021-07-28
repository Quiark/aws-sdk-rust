// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UntagResourceOutput {}
impl std::fmt::Debug for UntagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UntagResourceOutput");
        formatter.finish()
    }
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput)
pub mod untag_resource_output {
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput)
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput)
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TagResourceOutput {}
impl std::fmt::Debug for TagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TagResourceOutput");
        formatter.finish()
    }
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput)
pub mod tag_resource_output {
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput)
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput)
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForResourceOutput {
    /// <p>Information about the tags.</p>
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl std::fmt::Debug for ListTagsForResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForResourceOutput");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
pub mod list_tags_for_resource_output {
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    }
    impl Builder {
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput)
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeSavingsPlansOfferingsOutput {
    /// <p>Information about the Savings Plans offerings.</p>
    pub search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOffering>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more
    /// results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeSavingsPlansOfferingsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeSavingsPlansOfferingsOutput");
        formatter.field("search_results", &self.search_results);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeSavingsPlansOfferingsOutput`](crate::output::DescribeSavingsPlansOfferingsOutput)
pub mod describe_savings_plans_offerings_output {
    /// A builder for [`DescribeSavingsPlansOfferingsOutput`](crate::output::DescribeSavingsPlansOfferingsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) search_results:
            std::option::Option<std::vec::Vec<crate::model::SavingsPlanOffering>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn search_results(
            mut self,
            input: impl Into<crate::model::SavingsPlanOffering>,
        ) -> Self {
            let mut v = self.search_results.unwrap_or_default();
            v.push(input.into());
            self.search_results = Some(v);
            self
        }
        pub fn set_search_results(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOffering>>,
        ) -> Self {
            self.search_results = input;
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more
        /// results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeSavingsPlansOfferingsOutput`](crate::output::DescribeSavingsPlansOfferingsOutput)
        pub fn build(self) -> crate::output::DescribeSavingsPlansOfferingsOutput {
            crate::output::DescribeSavingsPlansOfferingsOutput {
                search_results: self.search_results,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeSavingsPlansOfferingsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansOfferingsOutput`](crate::output::DescribeSavingsPlansOfferingsOutput)
    pub fn builder() -> crate::output::describe_savings_plans_offerings_output::Builder {
        crate::output::describe_savings_plans_offerings_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeSavingsPlansOfferingRatesOutput {
    /// <p>Information about the Savings Plans offering rates.</p>
    pub search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOfferingRate>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more
    /// results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeSavingsPlansOfferingRatesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeSavingsPlansOfferingRatesOutput");
        formatter.field("search_results", &self.search_results);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeSavingsPlansOfferingRatesOutput`](crate::output::DescribeSavingsPlansOfferingRatesOutput)
pub mod describe_savings_plans_offering_rates_output {
    /// A builder for [`DescribeSavingsPlansOfferingRatesOutput`](crate::output::DescribeSavingsPlansOfferingRatesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) search_results:
            std::option::Option<std::vec::Vec<crate::model::SavingsPlanOfferingRate>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn search_results(
            mut self,
            input: impl Into<crate::model::SavingsPlanOfferingRate>,
        ) -> Self {
            let mut v = self.search_results.unwrap_or_default();
            v.push(input.into());
            self.search_results = Some(v);
            self
        }
        pub fn set_search_results(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SavingsPlanOfferingRate>>,
        ) -> Self {
            self.search_results = input;
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more
        /// results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeSavingsPlansOfferingRatesOutput`](crate::output::DescribeSavingsPlansOfferingRatesOutput)
        pub fn build(self) -> crate::output::DescribeSavingsPlansOfferingRatesOutput {
            crate::output::DescribeSavingsPlansOfferingRatesOutput {
                search_results: self.search_results,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeSavingsPlansOfferingRatesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansOfferingRatesOutput`](crate::output::DescribeSavingsPlansOfferingRatesOutput)
    pub fn builder() -> crate::output::describe_savings_plans_offering_rates_output::Builder {
        crate::output::describe_savings_plans_offering_rates_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeSavingsPlansOutput {
    /// <p>Information about the Savings Plans.</p>
    pub savings_plans: std::option::Option<std::vec::Vec<crate::model::SavingsPlan>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more
    /// results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeSavingsPlansOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeSavingsPlansOutput");
        formatter.field("savings_plans", &self.savings_plans);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput)
pub mod describe_savings_plans_output {
    /// A builder for [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) savings_plans: std::option::Option<std::vec::Vec<crate::model::SavingsPlan>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        pub fn savings_plans(mut self, input: impl Into<crate::model::SavingsPlan>) -> Self {
            let mut v = self.savings_plans.unwrap_or_default();
            v.push(input.into());
            self.savings_plans = Some(v);
            self
        }
        pub fn set_savings_plans(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SavingsPlan>>,
        ) -> Self {
            self.savings_plans = input;
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more
        /// results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput)
        pub fn build(self) -> crate::output::DescribeSavingsPlansOutput {
            crate::output::DescribeSavingsPlansOutput {
                savings_plans: self.savings_plans,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeSavingsPlansOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlansOutput`](crate::output::DescribeSavingsPlansOutput)
    pub fn builder() -> crate::output::describe_savings_plans_output::Builder {
        crate::output::describe_savings_plans_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeSavingsPlanRatesOutput {
    /// <p>The ID of the Savings Plan.</p>
    pub savings_plan_id: std::option::Option<std::string::String>,
    /// <p>Information about the Savings Plans rates.</p>
    pub search_results: std::option::Option<std::vec::Vec<crate::model::SavingsPlanRate>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more
    /// results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DescribeSavingsPlanRatesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeSavingsPlanRatesOutput");
        formatter.field("savings_plan_id", &self.savings_plan_id);
        formatter.field("search_results", &self.search_results);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeSavingsPlanRatesOutput`](crate::output::DescribeSavingsPlanRatesOutput)
pub mod describe_savings_plan_rates_output {
    /// A builder for [`DescribeSavingsPlanRatesOutput`](crate::output::DescribeSavingsPlanRatesOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) savings_plan_id: std::option::Option<std::string::String>,
        pub(crate) search_results:
            std::option::Option<std::vec::Vec<crate::model::SavingsPlanRate>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the Savings Plan.</p>
        pub fn savings_plan_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.savings_plan_id = Some(input.into());
            self
        }
        pub fn set_savings_plan_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.savings_plan_id = input;
            self
        }
        pub fn search_results(mut self, input: impl Into<crate::model::SavingsPlanRate>) -> Self {
            let mut v = self.search_results.unwrap_or_default();
            v.push(input.into());
            self.search_results = Some(v);
            self
        }
        pub fn set_search_results(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::SavingsPlanRate>>,
        ) -> Self {
            self.search_results = input;
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there are no more
        /// results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeSavingsPlanRatesOutput`](crate::output::DescribeSavingsPlanRatesOutput)
        pub fn build(self) -> crate::output::DescribeSavingsPlanRatesOutput {
            crate::output::DescribeSavingsPlanRatesOutput {
                savings_plan_id: self.savings_plan_id,
                search_results: self.search_results,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeSavingsPlanRatesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSavingsPlanRatesOutput`](crate::output::DescribeSavingsPlanRatesOutput)
    pub fn builder() -> crate::output::describe_savings_plan_rates_output::Builder {
        crate::output::describe_savings_plan_rates_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteQueuedSavingsPlanOutput {}
impl std::fmt::Debug for DeleteQueuedSavingsPlanOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteQueuedSavingsPlanOutput");
        formatter.finish()
    }
}
/// See [`DeleteQueuedSavingsPlanOutput`](crate::output::DeleteQueuedSavingsPlanOutput)
pub mod delete_queued_savings_plan_output {
    /// A builder for [`DeleteQueuedSavingsPlanOutput`](crate::output::DeleteQueuedSavingsPlanOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteQueuedSavingsPlanOutput`](crate::output::DeleteQueuedSavingsPlanOutput)
        pub fn build(self) -> crate::output::DeleteQueuedSavingsPlanOutput {
            crate::output::DeleteQueuedSavingsPlanOutput {}
        }
    }
}
impl DeleteQueuedSavingsPlanOutput {
    /// Creates a new builder-style object to manufacture [`DeleteQueuedSavingsPlanOutput`](crate::output::DeleteQueuedSavingsPlanOutput)
    pub fn builder() -> crate::output::delete_queued_savings_plan_output::Builder {
        crate::output::delete_queued_savings_plan_output::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateSavingsPlanOutput {
    /// <p>The ID of the Savings Plan.</p>
    pub savings_plan_id: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CreateSavingsPlanOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateSavingsPlanOutput");
        formatter.field("savings_plan_id", &self.savings_plan_id);
        formatter.finish()
    }
}
/// See [`CreateSavingsPlanOutput`](crate::output::CreateSavingsPlanOutput)
pub mod create_savings_plan_output {
    /// A builder for [`CreateSavingsPlanOutput`](crate::output::CreateSavingsPlanOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) savings_plan_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The ID of the Savings Plan.</p>
        pub fn savings_plan_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.savings_plan_id = Some(input.into());
            self
        }
        pub fn set_savings_plan_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.savings_plan_id = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateSavingsPlanOutput`](crate::output::CreateSavingsPlanOutput)
        pub fn build(self) -> crate::output::CreateSavingsPlanOutput {
            crate::output::CreateSavingsPlanOutput {
                savings_plan_id: self.savings_plan_id,
            }
        }
    }
}
impl CreateSavingsPlanOutput {
    /// Creates a new builder-style object to manufacture [`CreateSavingsPlanOutput`](crate::output::CreateSavingsPlanOutput)
    pub fn builder() -> crate::output::create_savings_plan_output::Builder {
        crate::output::create_savings_plan_output::Builder::default()
    }
}