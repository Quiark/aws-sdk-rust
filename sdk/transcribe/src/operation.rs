// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates a new custom language model. Use Amazon S3 prefixes to provide the location of your
/// input files. The time it takes to create your model depends on the size of your training
/// data.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateLanguageModel {
    _private: (),
}
impl CreateLanguageModel {
    /// Creates a new builder-style object to manufacture [`CreateLanguageModelInput`](crate::input::CreateLanguageModelInput)
    pub fn builder() -> crate::input::create_language_model_input::Builder {
        crate::input::create_language_model_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateLanguageModel {
    type Output = std::result::Result<
        crate::output::CreateLanguageModelOutput,
        crate::error::CreateLanguageModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_language_model_error(response)
        } else {
            crate::operation_deser::parse_create_language_model_response(response)
        }
    }
}

/// <p>Creates a new custom vocabulary that you can use to
/// change
/// how Amazon Transcribe Medical transcribes your audio file.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateMedicalVocabulary {
    _private: (),
}
impl CreateMedicalVocabulary {
    /// Creates a new builder-style object to manufacture [`CreateMedicalVocabularyInput`](crate::input::CreateMedicalVocabularyInput)
    pub fn builder() -> crate::input::create_medical_vocabulary_input::Builder {
        crate::input::create_medical_vocabulary_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateMedicalVocabulary {
    type Output = std::result::Result<
        crate::output::CreateMedicalVocabularyOutput,
        crate::error::CreateMedicalVocabularyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_medical_vocabulary_error(response)
        } else {
            crate::operation_deser::parse_create_medical_vocabulary_response(response)
        }
    }
}

/// <p>Creates a new custom vocabulary that you can use to change the way Amazon Transcribe handles
/// transcription of an audio file.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateVocabulary {
    _private: (),
}
impl CreateVocabulary {
    /// Creates a new builder-style object to manufacture [`CreateVocabularyInput`](crate::input::CreateVocabularyInput)
    pub fn builder() -> crate::input::create_vocabulary_input::Builder {
        crate::input::create_vocabulary_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateVocabulary {
    type Output = std::result::Result<
        crate::output::CreateVocabularyOutput,
        crate::error::CreateVocabularyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_vocabulary_error(response)
        } else {
            crate::operation_deser::parse_create_vocabulary_response(response)
        }
    }
}

/// <p>Creates a new vocabulary filter that you can use to filter words, such as profane
/// words, from the output of a transcription job.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateVocabularyFilter {
    _private: (),
}
impl CreateVocabularyFilter {
    /// Creates a new builder-style object to manufacture [`CreateVocabularyFilterInput`](crate::input::CreateVocabularyFilterInput)
    pub fn builder() -> crate::input::create_vocabulary_filter_input::Builder {
        crate::input::create_vocabulary_filter_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateVocabularyFilter {
    type Output = std::result::Result<
        crate::output::CreateVocabularyFilterOutput,
        crate::error::CreateVocabularyFilterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_vocabulary_filter_error(response)
        } else {
            crate::operation_deser::parse_create_vocabulary_filter_response(response)
        }
    }
}

/// <p>Deletes a custom language model using its name.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteLanguageModel {
    _private: (),
}
impl DeleteLanguageModel {
    /// Creates a new builder-style object to manufacture [`DeleteLanguageModelInput`](crate::input::DeleteLanguageModelInput)
    pub fn builder() -> crate::input::delete_language_model_input::Builder {
        crate::input::delete_language_model_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteLanguageModel {
    type Output = std::result::Result<
        crate::output::DeleteLanguageModelOutput,
        crate::error::DeleteLanguageModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_language_model_error(response)
        } else {
            crate::operation_deser::parse_delete_language_model_response(response)
        }
    }
}

/// <p>Deletes a transcription job generated by Amazon Transcribe Medical and any related information.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteMedicalTranscriptionJob {
    _private: (),
}
impl DeleteMedicalTranscriptionJob {
    /// Creates a new builder-style object to manufacture [`DeleteMedicalTranscriptionJobInput`](crate::input::DeleteMedicalTranscriptionJobInput)
    pub fn builder() -> crate::input::delete_medical_transcription_job_input::Builder {
        crate::input::delete_medical_transcription_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteMedicalTranscriptionJob {
    type Output = std::result::Result<
        crate::output::DeleteMedicalTranscriptionJobOutput,
        crate::error::DeleteMedicalTranscriptionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_medical_transcription_job_error(response)
        } else {
            crate::operation_deser::parse_delete_medical_transcription_job_response(response)
        }
    }
}

/// <p>Deletes a vocabulary from Amazon Transcribe Medical.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteMedicalVocabulary {
    _private: (),
}
impl DeleteMedicalVocabulary {
    /// Creates a new builder-style object to manufacture [`DeleteMedicalVocabularyInput`](crate::input::DeleteMedicalVocabularyInput)
    pub fn builder() -> crate::input::delete_medical_vocabulary_input::Builder {
        crate::input::delete_medical_vocabulary_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteMedicalVocabulary {
    type Output = std::result::Result<
        crate::output::DeleteMedicalVocabularyOutput,
        crate::error::DeleteMedicalVocabularyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_medical_vocabulary_error(response)
        } else {
            crate::operation_deser::parse_delete_medical_vocabulary_response(response)
        }
    }
}

/// <p>Deletes a previously submitted transcription job along with any other generated
/// results such as the transcription, models, and so on.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTranscriptionJob {
    _private: (),
}
impl DeleteTranscriptionJob {
    /// Creates a new builder-style object to manufacture [`DeleteTranscriptionJobInput`](crate::input::DeleteTranscriptionJobInput)
    pub fn builder() -> crate::input::delete_transcription_job_input::Builder {
        crate::input::delete_transcription_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteTranscriptionJob {
    type Output = std::result::Result<
        crate::output::DeleteTranscriptionJobOutput,
        crate::error::DeleteTranscriptionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_transcription_job_error(response)
        } else {
            crate::operation_deser::parse_delete_transcription_job_response(response)
        }
    }
}

/// <p>Deletes a vocabulary from Amazon Transcribe. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteVocabulary {
    _private: (),
}
impl DeleteVocabulary {
    /// Creates a new builder-style object to manufacture [`DeleteVocabularyInput`](crate::input::DeleteVocabularyInput)
    pub fn builder() -> crate::input::delete_vocabulary_input::Builder {
        crate::input::delete_vocabulary_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteVocabulary {
    type Output = std::result::Result<
        crate::output::DeleteVocabularyOutput,
        crate::error::DeleteVocabularyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_vocabulary_error(response)
        } else {
            crate::operation_deser::parse_delete_vocabulary_response(response)
        }
    }
}

/// <p>Removes a vocabulary filter.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteVocabularyFilter {
    _private: (),
}
impl DeleteVocabularyFilter {
    /// Creates a new builder-style object to manufacture [`DeleteVocabularyFilterInput`](crate::input::DeleteVocabularyFilterInput)
    pub fn builder() -> crate::input::delete_vocabulary_filter_input::Builder {
        crate::input::delete_vocabulary_filter_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteVocabularyFilter {
    type Output = std::result::Result<
        crate::output::DeleteVocabularyFilterOutput,
        crate::error::DeleteVocabularyFilterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_vocabulary_filter_error(response)
        } else {
            crate::operation_deser::parse_delete_vocabulary_filter_response(response)
        }
    }
}

/// <p>Gets information about a single custom language model. Use this information to see
/// details about the language model in your AWS account. You can also see whether the base
/// language model used to create your custom language model has been updated. If Amazon Transcribe has
/// updated the base model, you can create a new custom language model using the updated
/// base model. If the language model wasn't created, you can use this operation to
/// understand why Amazon Transcribe couldn't create it. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeLanguageModel {
    _private: (),
}
impl DescribeLanguageModel {
    /// Creates a new builder-style object to manufacture [`DescribeLanguageModelInput`](crate::input::DescribeLanguageModelInput)
    pub fn builder() -> crate::input::describe_language_model_input::Builder {
        crate::input::describe_language_model_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeLanguageModel {
    type Output = std::result::Result<
        crate::output::DescribeLanguageModelOutput,
        crate::error::DescribeLanguageModelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_language_model_error(response)
        } else {
            crate::operation_deser::parse_describe_language_model_response(response)
        }
    }
}

/// <p>Returns information about a transcription job from Amazon Transcribe Medical. To see the status of the
/// job, check the <code>TranscriptionJobStatus</code> field. If the status is
/// <code>COMPLETED</code>, the job is finished. You find the results of the completed
/// job in the <code>TranscriptFileUri</code> field.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetMedicalTranscriptionJob {
    _private: (),
}
impl GetMedicalTranscriptionJob {
    /// Creates a new builder-style object to manufacture [`GetMedicalTranscriptionJobInput`](crate::input::GetMedicalTranscriptionJobInput)
    pub fn builder() -> crate::input::get_medical_transcription_job_input::Builder {
        crate::input::get_medical_transcription_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetMedicalTranscriptionJob {
    type Output = std::result::Result<
        crate::output::GetMedicalTranscriptionJobOutput,
        crate::error::GetMedicalTranscriptionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_medical_transcription_job_error(response)
        } else {
            crate::operation_deser::parse_get_medical_transcription_job_response(response)
        }
    }
}

/// <p>Retrieves information about a medical vocabulary.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetMedicalVocabulary {
    _private: (),
}
impl GetMedicalVocabulary {
    /// Creates a new builder-style object to manufacture [`GetMedicalVocabularyInput`](crate::input::GetMedicalVocabularyInput)
    pub fn builder() -> crate::input::get_medical_vocabulary_input::Builder {
        crate::input::get_medical_vocabulary_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetMedicalVocabulary {
    type Output = std::result::Result<
        crate::output::GetMedicalVocabularyOutput,
        crate::error::GetMedicalVocabularyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_medical_vocabulary_error(response)
        } else {
            crate::operation_deser::parse_get_medical_vocabulary_response(response)
        }
    }
}

/// <p>Returns information about a transcription job. To see the status of the job, check the
/// <code>TranscriptionJobStatus</code> field. If the status is <code>COMPLETED</code>,
/// the job is finished and you can find the results at the location specified in the
/// <code>TranscriptFileUri</code> field. If you enable content redaction, the redacted
/// transcript appears in <code>RedactedTranscriptFileUri</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTranscriptionJob {
    _private: (),
}
impl GetTranscriptionJob {
    /// Creates a new builder-style object to manufacture [`GetTranscriptionJobInput`](crate::input::GetTranscriptionJobInput)
    pub fn builder() -> crate::input::get_transcription_job_input::Builder {
        crate::input::get_transcription_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetTranscriptionJob {
    type Output = std::result::Result<
        crate::output::GetTranscriptionJobOutput,
        crate::error::GetTranscriptionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_transcription_job_error(response)
        } else {
            crate::operation_deser::parse_get_transcription_job_response(response)
        }
    }
}

/// <p>Gets information about a vocabulary. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetVocabulary {
    _private: (),
}
impl GetVocabulary {
    /// Creates a new builder-style object to manufacture [`GetVocabularyInput`](crate::input::GetVocabularyInput)
    pub fn builder() -> crate::input::get_vocabulary_input::Builder {
        crate::input::get_vocabulary_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetVocabulary {
    type Output =
        std::result::Result<crate::output::GetVocabularyOutput, crate::error::GetVocabularyError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_vocabulary_error(response)
        } else {
            crate::operation_deser::parse_get_vocabulary_response(response)
        }
    }
}

/// <p>Returns information about a vocabulary filter.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetVocabularyFilter {
    _private: (),
}
impl GetVocabularyFilter {
    /// Creates a new builder-style object to manufacture [`GetVocabularyFilterInput`](crate::input::GetVocabularyFilterInput)
    pub fn builder() -> crate::input::get_vocabulary_filter_input::Builder {
        crate::input::get_vocabulary_filter_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetVocabularyFilter {
    type Output = std::result::Result<
        crate::output::GetVocabularyFilterOutput,
        crate::error::GetVocabularyFilterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_vocabulary_filter_error(response)
        } else {
            crate::operation_deser::parse_get_vocabulary_filter_response(response)
        }
    }
}

/// <p>Provides more information about the custom language models you've created. You can use
/// the information in this list to find a specific custom language model. You can then use
/// the  operation to get more information about
/// it.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListLanguageModels {
    _private: (),
}
impl ListLanguageModels {
    /// Creates a new builder-style object to manufacture [`ListLanguageModelsInput`](crate::input::ListLanguageModelsInput)
    pub fn builder() -> crate::input::list_language_models_input::Builder {
        crate::input::list_language_models_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListLanguageModels {
    type Output = std::result::Result<
        crate::output::ListLanguageModelsOutput,
        crate::error::ListLanguageModelsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_language_models_error(response)
        } else {
            crate::operation_deser::parse_list_language_models_response(response)
        }
    }
}

/// <p>Lists medical transcription jobs with a specified status or substring that matches
/// their names.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListMedicalTranscriptionJobs {
    _private: (),
}
impl ListMedicalTranscriptionJobs {
    /// Creates a new builder-style object to manufacture [`ListMedicalTranscriptionJobsInput`](crate::input::ListMedicalTranscriptionJobsInput)
    pub fn builder() -> crate::input::list_medical_transcription_jobs_input::Builder {
        crate::input::list_medical_transcription_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListMedicalTranscriptionJobs {
    type Output = std::result::Result<
        crate::output::ListMedicalTranscriptionJobsOutput,
        crate::error::ListMedicalTranscriptionJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_medical_transcription_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_medical_transcription_jobs_response(response)
        }
    }
}

/// <p>Returns a list of vocabularies that match the specified criteria. If you don't enter a
/// value in any of the request parameters, returns the entire list of vocabularies.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListMedicalVocabularies {
    _private: (),
}
impl ListMedicalVocabularies {
    /// Creates a new builder-style object to manufacture [`ListMedicalVocabulariesInput`](crate::input::ListMedicalVocabulariesInput)
    pub fn builder() -> crate::input::list_medical_vocabularies_input::Builder {
        crate::input::list_medical_vocabularies_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListMedicalVocabularies {
    type Output = std::result::Result<
        crate::output::ListMedicalVocabulariesOutput,
        crate::error::ListMedicalVocabulariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_medical_vocabularies_error(response)
        } else {
            crate::operation_deser::parse_list_medical_vocabularies_response(response)
        }
    }
}

/// <p>Lists transcription jobs with the specified status.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTranscriptionJobs {
    _private: (),
}
impl ListTranscriptionJobs {
    /// Creates a new builder-style object to manufacture [`ListTranscriptionJobsInput`](crate::input::ListTranscriptionJobsInput)
    pub fn builder() -> crate::input::list_transcription_jobs_input::Builder {
        crate::input::list_transcription_jobs_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTranscriptionJobs {
    type Output = std::result::Result<
        crate::output::ListTranscriptionJobsOutput,
        crate::error::ListTranscriptionJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_transcription_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_transcription_jobs_response(response)
        }
    }
}

/// <p>Returns a list of vocabularies that match the specified criteria. If no criteria are
/// specified, returns the entire list of vocabularies.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListVocabularies {
    _private: (),
}
impl ListVocabularies {
    /// Creates a new builder-style object to manufacture [`ListVocabulariesInput`](crate::input::ListVocabulariesInput)
    pub fn builder() -> crate::input::list_vocabularies_input::Builder {
        crate::input::list_vocabularies_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListVocabularies {
    type Output = std::result::Result<
        crate::output::ListVocabulariesOutput,
        crate::error::ListVocabulariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_vocabularies_error(response)
        } else {
            crate::operation_deser::parse_list_vocabularies_response(response)
        }
    }
}

/// <p>Gets information about vocabulary filters.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListVocabularyFilters {
    _private: (),
}
impl ListVocabularyFilters {
    /// Creates a new builder-style object to manufacture [`ListVocabularyFiltersInput`](crate::input::ListVocabularyFiltersInput)
    pub fn builder() -> crate::input::list_vocabulary_filters_input::Builder {
        crate::input::list_vocabulary_filters_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListVocabularyFilters {
    type Output = std::result::Result<
        crate::output::ListVocabularyFiltersOutput,
        crate::error::ListVocabularyFiltersError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_vocabulary_filters_error(response)
        } else {
            crate::operation_deser::parse_list_vocabulary_filters_response(response)
        }
    }
}

/// <p>Starts a batch job to transcribe medical speech to text.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartMedicalTranscriptionJob {
    _private: (),
}
impl StartMedicalTranscriptionJob {
    /// Creates a new builder-style object to manufacture [`StartMedicalTranscriptionJobInput`](crate::input::StartMedicalTranscriptionJobInput)
    pub fn builder() -> crate::input::start_medical_transcription_job_input::Builder {
        crate::input::start_medical_transcription_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartMedicalTranscriptionJob {
    type Output = std::result::Result<
        crate::output::StartMedicalTranscriptionJobOutput,
        crate::error::StartMedicalTranscriptionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_medical_transcription_job_error(response)
        } else {
            crate::operation_deser::parse_start_medical_transcription_job_response(response)
        }
    }
}

/// <p>Starts an asynchronous job to transcribe speech to text.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartTranscriptionJob {
    _private: (),
}
impl StartTranscriptionJob {
    /// Creates a new builder-style object to manufacture [`StartTranscriptionJobInput`](crate::input::StartTranscriptionJobInput)
    pub fn builder() -> crate::input::start_transcription_job_input::Builder {
        crate::input::start_transcription_job_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for StartTranscriptionJob {
    type Output = std::result::Result<
        crate::output::StartTranscriptionJobOutput,
        crate::error::StartTranscriptionJobError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_transcription_job_error(response)
        } else {
            crate::operation_deser::parse_start_transcription_job_response(response)
        }
    }
}

/// <p>Updates a vocabulary with new values that you provide in a different text file from
/// the one you used to create the vocabulary. The <code>UpdateMedicalVocabulary</code>
/// operation overwrites all of the existing information with the values that you provide in
/// the request.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateMedicalVocabulary {
    _private: (),
}
impl UpdateMedicalVocabulary {
    /// Creates a new builder-style object to manufacture [`UpdateMedicalVocabularyInput`](crate::input::UpdateMedicalVocabularyInput)
    pub fn builder() -> crate::input::update_medical_vocabulary_input::Builder {
        crate::input::update_medical_vocabulary_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateMedicalVocabulary {
    type Output = std::result::Result<
        crate::output::UpdateMedicalVocabularyOutput,
        crate::error::UpdateMedicalVocabularyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_medical_vocabulary_error(response)
        } else {
            crate::operation_deser::parse_update_medical_vocabulary_response(response)
        }
    }
}

/// <p>Updates an existing vocabulary with new values. The <code>UpdateVocabulary</code>
/// operation overwrites all of the existing information with the values that you provide in
/// the request. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateVocabulary {
    _private: (),
}
impl UpdateVocabulary {
    /// Creates a new builder-style object to manufacture [`UpdateVocabularyInput`](crate::input::UpdateVocabularyInput)
    pub fn builder() -> crate::input::update_vocabulary_input::Builder {
        crate::input::update_vocabulary_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateVocabulary {
    type Output = std::result::Result<
        crate::output::UpdateVocabularyOutput,
        crate::error::UpdateVocabularyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_vocabulary_error(response)
        } else {
            crate::operation_deser::parse_update_vocabulary_response(response)
        }
    }
}

/// <p>Updates a vocabulary filter with a new list of filtered words.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateVocabularyFilter {
    _private: (),
}
impl UpdateVocabularyFilter {
    /// Creates a new builder-style object to manufacture [`UpdateVocabularyFilterInput`](crate::input::UpdateVocabularyFilterInput)
    pub fn builder() -> crate::input::update_vocabulary_filter_input::Builder {
        crate::input::update_vocabulary_filter_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateVocabularyFilter {
    type Output = std::result::Result<
        crate::output::UpdateVocabularyFilterOutput,
        crate::error::UpdateVocabularyFilterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_vocabulary_filter_error(response)
        } else {
            crate::operation_deser::parse_update_vocabulary_filter_response(response)
        }
    }
}