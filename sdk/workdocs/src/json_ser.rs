// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_add_resource_permissions_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddResourcePermissionsInput,
) {
    if let Some(var_1) = &input.notification_options {
        let mut object_2 = object.key("NotificationOptions").start_object();
        crate::json_ser::serialize_structure_notification_options(&mut object_2, var_1);
        object_2.finish();
    }
    if let Some(var_3) = &input.principals {
        let mut array_4 = object.key("Principals").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_share_principal(&mut object_6, item_5);
                object_6.finish();
            }
        }
        array_4.finish();
    }
}

pub fn serialize_structure_create_comment_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCommentInput,
) {
    if input.notify_collaborators {
        object
            .key("NotifyCollaborators")
            .boolean(input.notify_collaborators);
    }
    if let Some(var_7) = &input.parent_id {
        object.key("ParentId").string(var_7);
    }
    if let Some(var_8) = &input.text {
        object.key("Text").string(var_8);
    }
    if let Some(var_9) = &input.thread_id {
        object.key("ThreadId").string(var_9);
    }
    if let Some(var_10) = &input.visibility {
        object.key("Visibility").string(var_10.as_str());
    }
}

pub fn serialize_structure_create_custom_metadata_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCustomMetadataInput,
) {
    if let Some(var_11) = &input.custom_metadata {
        let mut object_12 = object.key("CustomMetadata").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13).string(value_14);
            }
        }
        object_12.finish();
    }
}

pub fn serialize_structure_create_folder_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFolderInput,
) {
    if let Some(var_15) = &input.name {
        object.key("Name").string(var_15);
    }
    if let Some(var_16) = &input.parent_folder_id {
        object.key("ParentFolderId").string(var_16);
    }
}

pub fn serialize_structure_create_labels_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLabelsInput,
) {
    if let Some(var_17) = &input.labels {
        let mut array_18 = object.key("Labels").start_array();
        for item_19 in var_17 {
            {
                array_18.value().string(item_19);
            }
        }
        array_18.finish();
    }
}

pub fn serialize_structure_create_notification_subscription_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNotificationSubscriptionInput,
) {
    if let Some(var_20) = &input.endpoint {
        object.key("Endpoint").string(var_20);
    }
    if let Some(var_21) = &input.protocol {
        object.key("Protocol").string(var_21.as_str());
    }
    if let Some(var_22) = &input.subscription_type {
        object.key("SubscriptionType").string(var_22.as_str());
    }
}

pub fn serialize_structure_create_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateUserInput,
) {
    if let Some(var_23) = &input.email_address {
        object.key("EmailAddress").string(var_23);
    }
    if let Some(var_24) = &input.given_name {
        object.key("GivenName").string(var_24);
    }
    if let Some(var_25) = &input.organization_id {
        object.key("OrganizationId").string(var_25);
    }
    if let Some(var_26) = &input.password {
        object.key("Password").string(var_26);
    }
    if let Some(var_27) = &input.storage_rule {
        let mut object_28 = object.key("StorageRule").start_object();
        crate::json_ser::serialize_structure_storage_rule_type(&mut object_28, var_27);
        object_28.finish();
    }
    if let Some(var_29) = &input.surname {
        object.key("Surname").string(var_29);
    }
    if let Some(var_30) = &input.time_zone_id {
        object.key("TimeZoneId").string(var_30);
    }
    if let Some(var_31) = &input.username {
        object.key("Username").string(var_31);
    }
}

pub fn serialize_structure_initiate_document_version_upload_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::InitiateDocumentVersionUploadInput,
) {
    if let Some(var_32) = &input.content_created_timestamp {
        object
            .key("ContentCreatedTimestamp")
            .instant(var_32, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_33) = &input.content_modified_timestamp {
        object
            .key("ContentModifiedTimestamp")
            .instant(var_33, smithy_types::instant::Format::EpochSeconds);
    }
    if let Some(var_34) = &input.content_type {
        object.key("ContentType").string(var_34);
    }
    if let Some(var_35) = &input.document_size_in_bytes {
        object.key("DocumentSizeInBytes").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_35).into()),
        );
    }
    if let Some(var_36) = &input.id {
        object.key("Id").string(var_36);
    }
    if let Some(var_37) = &input.name {
        object.key("Name").string(var_37);
    }
    if let Some(var_38) = &input.parent_folder_id {
        object.key("ParentFolderId").string(var_38);
    }
}

pub fn serialize_structure_update_document_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDocumentInput,
) {
    if let Some(var_39) = &input.name {
        object.key("Name").string(var_39);
    }
    if let Some(var_40) = &input.parent_folder_id {
        object.key("ParentFolderId").string(var_40);
    }
    if let Some(var_41) = &input.resource_state {
        object.key("ResourceState").string(var_41.as_str());
    }
}

pub fn serialize_structure_update_document_version_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDocumentVersionInput,
) {
    if let Some(var_42) = &input.version_status {
        object.key("VersionStatus").string(var_42.as_str());
    }
}

pub fn serialize_structure_update_folder_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFolderInput,
) {
    if let Some(var_43) = &input.name {
        object.key("Name").string(var_43);
    }
    if let Some(var_44) = &input.parent_folder_id {
        object.key("ParentFolderId").string(var_44);
    }
    if let Some(var_45) = &input.resource_state {
        object.key("ResourceState").string(var_45.as_str());
    }
}

pub fn serialize_structure_update_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateUserInput,
) {
    if let Some(var_46) = &input.given_name {
        object.key("GivenName").string(var_46);
    }
    if let Some(var_47) = &input.grant_poweruser_privileges {
        object
            .key("GrantPoweruserPrivileges")
            .string(var_47.as_str());
    }
    if let Some(var_48) = &input.locale {
        object.key("Locale").string(var_48.as_str());
    }
    if let Some(var_49) = &input.storage_rule {
        let mut object_50 = object.key("StorageRule").start_object();
        crate::json_ser::serialize_structure_storage_rule_type(&mut object_50, var_49);
        object_50.finish();
    }
    if let Some(var_51) = &input.surname {
        object.key("Surname").string(var_51);
    }
    if let Some(var_52) = &input.time_zone_id {
        object.key("TimeZoneId").string(var_52);
    }
    if let Some(var_53) = &input.r#type {
        object.key("Type").string(var_53.as_str());
    }
}

pub fn serialize_structure_notification_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationOptions,
) {
    if input.send_email {
        object.key("SendEmail").boolean(input.send_email);
    }
    if let Some(var_54) = &input.email_message {
        object.key("EmailMessage").string(var_54);
    }
}

pub fn serialize_structure_share_principal(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SharePrincipal,
) {
    if let Some(var_55) = &input.id {
        object.key("Id").string(var_55);
    }
    if let Some(var_56) = &input.r#type {
        object.key("Type").string(var_56.as_str());
    }
    if let Some(var_57) = &input.role {
        object.key("Role").string(var_57.as_str());
    }
}

pub fn serialize_structure_storage_rule_type(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StorageRuleType,
) {
    if let Some(var_58) = &input.storage_allocated_in_bytes {
        object.key("StorageAllocatedInBytes").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_58).into()),
        );
    }
    if let Some(var_59) = &input.storage_type {
        object.key("StorageType").string(var_59.as_str());
    }
}