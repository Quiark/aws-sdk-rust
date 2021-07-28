// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_add_tags(
    input: &crate::input::AddTagsInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(&mut out, "AddTags", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LoadBalancerNames");
    if let Some(var_2) = &input.load_balancer_names {
        let mut list_4 = scope_1.start_list(false, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("Tags");
    if let Some(var_7) = &input.tags {
        let mut list_9 = scope_6.start_list(false, None);
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            crate::query_ser::serialize_structure_tag(entry_10, item_8);
        }
        list_9.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_apply_security_groups_to_load_balancer(
    input: &crate::input::ApplySecurityGroupsToLoadBalancerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "ApplySecurityGroupsToLoadBalancer", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("LoadBalancerName");
    if let Some(var_12) = &input.load_balancer_name {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("SecurityGroups");
    if let Some(var_14) = &input.security_groups {
        let mut list_16 = scope_13.start_list(false, None);
        for item_15 in var_14 {
            #[allow(unused_mut)]
            let mut entry_17 = list_16.entry();
            entry_17.string(item_15);
        }
        list_16.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_attach_load_balancer_to_subnets(
    input: &crate::input::AttachLoadBalancerToSubnetsInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "AttachLoadBalancerToSubnets", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("LoadBalancerName");
    if let Some(var_19) = &input.load_balancer_name {
        scope_18.string(var_19);
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("Subnets");
    if let Some(var_21) = &input.subnets {
        let mut list_23 = scope_20.start_list(false, None);
        for item_22 in var_21 {
            #[allow(unused_mut)]
            let mut entry_24 = list_23.entry();
            entry_24.string(item_22);
        }
        list_23.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_configure_health_check(
    input: &crate::input::ConfigureHealthCheckInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(&mut out, "ConfigureHealthCheck", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("LoadBalancerName");
    if let Some(var_26) = &input.load_balancer_name {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("HealthCheck");
    if let Some(var_28) = &input.health_check {
        crate::query_ser::serialize_structure_health_check(scope_27, var_28);
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_app_cookie_stickiness_policy(
    input: &crate::input::CreateAppCookieStickinessPolicyInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "CreateAppCookieStickinessPolicy", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("LoadBalancerName");
    if let Some(var_30) = &input.load_balancer_name {
        scope_29.string(var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("PolicyName");
    if let Some(var_32) = &input.policy_name {
        scope_31.string(var_32);
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("CookieName");
    if let Some(var_34) = &input.cookie_name {
        scope_33.string(var_34);
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_lb_cookie_stickiness_policy(
    input: &crate::input::CreateLbCookieStickinessPolicyInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "CreateLBCookieStickinessPolicy", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("LoadBalancerName");
    if let Some(var_36) = &input.load_balancer_name {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("PolicyName");
    if let Some(var_38) = &input.policy_name {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("CookieExpirationPeriod");
    if let Some(var_40) = &input.cookie_expiration_period {
        scope_39.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_40).into()),
        );
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_load_balancer(
    input: &crate::input::CreateLoadBalancerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(&mut out, "CreateLoadBalancer", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("LoadBalancerName");
    if let Some(var_42) = &input.load_balancer_name {
        scope_41.string(var_42);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("Listeners");
    if let Some(var_44) = &input.listeners {
        let mut list_46 = scope_43.start_list(false, None);
        for item_45 in var_44 {
            #[allow(unused_mut)]
            let mut entry_47 = list_46.entry();
            crate::query_ser::serialize_structure_listener(entry_47, item_45);
        }
        list_46.finish();
    }
    #[allow(unused_mut)]
    let mut scope_48 = writer.prefix("AvailabilityZones");
    if let Some(var_49) = &input.availability_zones {
        let mut list_51 = scope_48.start_list(false, None);
        for item_50 in var_49 {
            #[allow(unused_mut)]
            let mut entry_52 = list_51.entry();
            entry_52.string(item_50);
        }
        list_51.finish();
    }
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("Subnets");
    if let Some(var_54) = &input.subnets {
        let mut list_56 = scope_53.start_list(false, None);
        for item_55 in var_54 {
            #[allow(unused_mut)]
            let mut entry_57 = list_56.entry();
            entry_57.string(item_55);
        }
        list_56.finish();
    }
    #[allow(unused_mut)]
    let mut scope_58 = writer.prefix("SecurityGroups");
    if let Some(var_59) = &input.security_groups {
        let mut list_61 = scope_58.start_list(false, None);
        for item_60 in var_59 {
            #[allow(unused_mut)]
            let mut entry_62 = list_61.entry();
            entry_62.string(item_60);
        }
        list_61.finish();
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("Scheme");
    if let Some(var_64) = &input.scheme {
        scope_63.string(var_64);
    }
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("Tags");
    if let Some(var_66) = &input.tags {
        let mut list_68 = scope_65.start_list(false, None);
        for item_67 in var_66 {
            #[allow(unused_mut)]
            let mut entry_69 = list_68.entry();
            crate::query_ser::serialize_structure_tag(entry_69, item_67);
        }
        list_68.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_load_balancer_listeners(
    input: &crate::input::CreateLoadBalancerListenersInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "CreateLoadBalancerListeners", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_70 = writer.prefix("LoadBalancerName");
    if let Some(var_71) = &input.load_balancer_name {
        scope_70.string(var_71);
    }
    #[allow(unused_mut)]
    let mut scope_72 = writer.prefix("Listeners");
    if let Some(var_73) = &input.listeners {
        let mut list_75 = scope_72.start_list(false, None);
        for item_74 in var_73 {
            #[allow(unused_mut)]
            let mut entry_76 = list_75.entry();
            crate::query_ser::serialize_structure_listener(entry_76, item_74);
        }
        list_75.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_create_load_balancer_policy(
    input: &crate::input::CreateLoadBalancerPolicyInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "CreateLoadBalancerPolicy", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("LoadBalancerName");
    if let Some(var_78) = &input.load_balancer_name {
        scope_77.string(var_78);
    }
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("PolicyName");
    if let Some(var_80) = &input.policy_name {
        scope_79.string(var_80);
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("PolicyTypeName");
    if let Some(var_82) = &input.policy_type_name {
        scope_81.string(var_82);
    }
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("PolicyAttributes");
    if let Some(var_84) = &input.policy_attributes {
        let mut list_86 = scope_83.start_list(false, None);
        for item_85 in var_84 {
            #[allow(unused_mut)]
            let mut entry_87 = list_86.entry();
            crate::query_ser::serialize_structure_policy_attribute(entry_87, item_85);
        }
        list_86.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_delete_load_balancer(
    input: &crate::input::DeleteLoadBalancerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(&mut out, "DeleteLoadBalancer", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_88 = writer.prefix("LoadBalancerName");
    if let Some(var_89) = &input.load_balancer_name {
        scope_88.string(var_89);
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_delete_load_balancer_listeners(
    input: &crate::input::DeleteLoadBalancerListenersInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DeleteLoadBalancerListeners", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_90 = writer.prefix("LoadBalancerName");
    if let Some(var_91) = &input.load_balancer_name {
        scope_90.string(var_91);
    }
    #[allow(unused_mut)]
    let mut scope_92 = writer.prefix("LoadBalancerPorts");
    if let Some(var_93) = &input.load_balancer_ports {
        let mut list_95 = scope_92.start_list(false, None);
        for item_94 in var_93 {
            #[allow(unused_mut)]
            let mut entry_96 = list_95.entry();
            entry_96.number(
                #[allow(clippy::useless_conversion)]
                smithy_types::Number::NegInt((*item_94).into()),
            );
        }
        list_95.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_delete_load_balancer_policy(
    input: &crate::input::DeleteLoadBalancerPolicyInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DeleteLoadBalancerPolicy", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_97 = writer.prefix("LoadBalancerName");
    if let Some(var_98) = &input.load_balancer_name {
        scope_97.string(var_98);
    }
    #[allow(unused_mut)]
    let mut scope_99 = writer.prefix("PolicyName");
    if let Some(var_100) = &input.policy_name {
        scope_99.string(var_100);
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_deregister_instances_from_load_balancer(
    input: &crate::input::DeregisterInstancesFromLoadBalancerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(
        &mut out,
        "DeregisterInstancesFromLoadBalancer",
        "2012-06-01",
    );
    #[allow(unused_mut)]
    let mut scope_101 = writer.prefix("LoadBalancerName");
    if let Some(var_102) = &input.load_balancer_name {
        scope_101.string(var_102);
    }
    #[allow(unused_mut)]
    let mut scope_103 = writer.prefix("Instances");
    if let Some(var_104) = &input.instances {
        let mut list_106 = scope_103.start_list(false, None);
        for item_105 in var_104 {
            #[allow(unused_mut)]
            let mut entry_107 = list_106.entry();
            crate::query_ser::serialize_structure_instance(entry_107, item_105);
        }
        list_106.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_account_limits(
    input: &crate::input::DescribeAccountLimitsInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DescribeAccountLimits", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_108 = writer.prefix("Marker");
    if let Some(var_109) = &input.marker {
        scope_108.string(var_109);
    }
    #[allow(unused_mut)]
    let mut scope_110 = writer.prefix("PageSize");
    if let Some(var_111) = &input.page_size {
        scope_110.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_111).into()),
        );
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_instance_health(
    input: &crate::input::DescribeInstanceHealthInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DescribeInstanceHealth", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_112 = writer.prefix("LoadBalancerName");
    if let Some(var_113) = &input.load_balancer_name {
        scope_112.string(var_113);
    }
    #[allow(unused_mut)]
    let mut scope_114 = writer.prefix("Instances");
    if let Some(var_115) = &input.instances {
        let mut list_117 = scope_114.start_list(false, None);
        for item_116 in var_115 {
            #[allow(unused_mut)]
            let mut entry_118 = list_117.entry();
            crate::query_ser::serialize_structure_instance(entry_118, item_116);
        }
        list_117.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_load_balancer_attributes(
    input: &crate::input::DescribeLoadBalancerAttributesInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DescribeLoadBalancerAttributes", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_119 = writer.prefix("LoadBalancerName");
    if let Some(var_120) = &input.load_balancer_name {
        scope_119.string(var_120);
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_load_balancer_policies(
    input: &crate::input::DescribeLoadBalancerPoliciesInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DescribeLoadBalancerPolicies", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_121 = writer.prefix("LoadBalancerName");
    if let Some(var_122) = &input.load_balancer_name {
        scope_121.string(var_122);
    }
    #[allow(unused_mut)]
    let mut scope_123 = writer.prefix("PolicyNames");
    if let Some(var_124) = &input.policy_names {
        let mut list_126 = scope_123.start_list(false, None);
        for item_125 in var_124 {
            #[allow(unused_mut)]
            let mut entry_127 = list_126.entry();
            entry_127.string(item_125);
        }
        list_126.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_load_balancer_policy_types(
    input: &crate::input::DescribeLoadBalancerPolicyTypesInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DescribeLoadBalancerPolicyTypes", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_128 = writer.prefix("PolicyTypeNames");
    if let Some(var_129) = &input.policy_type_names {
        let mut list_131 = scope_128.start_list(false, None);
        for item_130 in var_129 {
            #[allow(unused_mut)]
            let mut entry_132 = list_131.entry();
            entry_132.string(item_130);
        }
        list_131.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_load_balancers(
    input: &crate::input::DescribeLoadBalancersInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DescribeLoadBalancers", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_133 = writer.prefix("LoadBalancerNames");
    if let Some(var_134) = &input.load_balancer_names {
        let mut list_136 = scope_133.start_list(false, None);
        for item_135 in var_134 {
            #[allow(unused_mut)]
            let mut entry_137 = list_136.entry();
            entry_137.string(item_135);
        }
        list_136.finish();
    }
    #[allow(unused_mut)]
    let mut scope_138 = writer.prefix("Marker");
    if let Some(var_139) = &input.marker {
        scope_138.string(var_139);
    }
    #[allow(unused_mut)]
    let mut scope_140 = writer.prefix("PageSize");
    if let Some(var_141) = &input.page_size {
        scope_140.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_141).into()),
        );
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_describe_tags(
    input: &crate::input::DescribeTagsInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(&mut out, "DescribeTags", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_142 = writer.prefix("LoadBalancerNames");
    if let Some(var_143) = &input.load_balancer_names {
        let mut list_145 = scope_142.start_list(false, None);
        for item_144 in var_143 {
            #[allow(unused_mut)]
            let mut entry_146 = list_145.entry();
            entry_146.string(item_144);
        }
        list_145.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_detach_load_balancer_from_subnets(
    input: &crate::input::DetachLoadBalancerFromSubnetsInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "DetachLoadBalancerFromSubnets", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_147 = writer.prefix("LoadBalancerName");
    if let Some(var_148) = &input.load_balancer_name {
        scope_147.string(var_148);
    }
    #[allow(unused_mut)]
    let mut scope_149 = writer.prefix("Subnets");
    if let Some(var_150) = &input.subnets {
        let mut list_152 = scope_149.start_list(false, None);
        for item_151 in var_150 {
            #[allow(unused_mut)]
            let mut entry_153 = list_152.entry();
            entry_153.string(item_151);
        }
        list_152.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_disable_availability_zones_for_load_balancer(
    input: &crate::input::DisableAvailabilityZonesForLoadBalancerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(
        &mut out,
        "DisableAvailabilityZonesForLoadBalancer",
        "2012-06-01",
    );
    #[allow(unused_mut)]
    let mut scope_154 = writer.prefix("LoadBalancerName");
    if let Some(var_155) = &input.load_balancer_name {
        scope_154.string(var_155);
    }
    #[allow(unused_mut)]
    let mut scope_156 = writer.prefix("AvailabilityZones");
    if let Some(var_157) = &input.availability_zones {
        let mut list_159 = scope_156.start_list(false, None);
        for item_158 in var_157 {
            #[allow(unused_mut)]
            let mut entry_160 = list_159.entry();
            entry_160.string(item_158);
        }
        list_159.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_enable_availability_zones_for_load_balancer(
    input: &crate::input::EnableAvailabilityZonesForLoadBalancerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(
        &mut out,
        "EnableAvailabilityZonesForLoadBalancer",
        "2012-06-01",
    );
    #[allow(unused_mut)]
    let mut scope_161 = writer.prefix("LoadBalancerName");
    if let Some(var_162) = &input.load_balancer_name {
        scope_161.string(var_162);
    }
    #[allow(unused_mut)]
    let mut scope_163 = writer.prefix("AvailabilityZones");
    if let Some(var_164) = &input.availability_zones {
        let mut list_166 = scope_163.start_list(false, None);
        for item_165 in var_164 {
            #[allow(unused_mut)]
            let mut entry_167 = list_166.entry();
            entry_167.string(item_165);
        }
        list_166.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_modify_load_balancer_attributes(
    input: &crate::input::ModifyLoadBalancerAttributesInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "ModifyLoadBalancerAttributes", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_168 = writer.prefix("LoadBalancerName");
    if let Some(var_169) = &input.load_balancer_name {
        scope_168.string(var_169);
    }
    #[allow(unused_mut)]
    let mut scope_170 = writer.prefix("LoadBalancerAttributes");
    if let Some(var_171) = &input.load_balancer_attributes {
        crate::query_ser::serialize_structure_load_balancer_attributes(scope_170, var_171);
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_register_instances_with_load_balancer(
    input: &crate::input::RegisterInstancesWithLoadBalancerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "RegisterInstancesWithLoadBalancer", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_172 = writer.prefix("LoadBalancerName");
    if let Some(var_173) = &input.load_balancer_name {
        scope_172.string(var_173);
    }
    #[allow(unused_mut)]
    let mut scope_174 = writer.prefix("Instances");
    if let Some(var_175) = &input.instances {
        let mut list_177 = scope_174.start_list(false, None);
        for item_176 in var_175 {
            #[allow(unused_mut)]
            let mut entry_178 = list_177.entry();
            crate::query_ser::serialize_structure_instance(entry_178, item_176);
        }
        list_177.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_remove_tags(
    input: &crate::input::RemoveTagsInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(&mut out, "RemoveTags", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_179 = writer.prefix("LoadBalancerNames");
    if let Some(var_180) = &input.load_balancer_names {
        let mut list_182 = scope_179.start_list(false, None);
        for item_181 in var_180 {
            #[allow(unused_mut)]
            let mut entry_183 = list_182.entry();
            entry_183.string(item_181);
        }
        list_182.finish();
    }
    #[allow(unused_mut)]
    let mut scope_184 = writer.prefix("Tags");
    if let Some(var_185) = &input.tags {
        let mut list_187 = scope_184.start_list(false, None);
        for item_186 in var_185 {
            #[allow(unused_mut)]
            let mut entry_188 = list_187.entry();
            crate::query_ser::serialize_structure_tag_key_only(entry_188, item_186);
        }
        list_187.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_set_load_balancer_listener_ssl_certificate(
    input: &crate::input::SetLoadBalancerListenerSslCertificateInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(
        &mut out,
        "SetLoadBalancerListenerSSLCertificate",
        "2012-06-01",
    );
    #[allow(unused_mut)]
    let mut scope_189 = writer.prefix("LoadBalancerName");
    if let Some(var_190) = &input.load_balancer_name {
        scope_189.string(var_190);
    }
    #[allow(unused_mut)]
    let mut scope_191 = writer.prefix("LoadBalancerPort");
    {
        scope_191.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.load_balancer_port).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_192 = writer.prefix("SSLCertificateId");
    if let Some(var_193) = &input.ssl_certificate_id {
        scope_192.string(var_193);
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_set_load_balancer_policies_for_backend_server(
    input: &crate::input::SetLoadBalancerPoliciesForBackendServerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = smithy_query::QueryWriter::new(
        &mut out,
        "SetLoadBalancerPoliciesForBackendServer",
        "2012-06-01",
    );
    #[allow(unused_mut)]
    let mut scope_194 = writer.prefix("LoadBalancerName");
    if let Some(var_195) = &input.load_balancer_name {
        scope_194.string(var_195);
    }
    #[allow(unused_mut)]
    let mut scope_196 = writer.prefix("InstancePort");
    if let Some(var_197) = &input.instance_port {
        scope_196.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_197).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_198 = writer.prefix("PolicyNames");
    if let Some(var_199) = &input.policy_names {
        let mut list_201 = scope_198.start_list(false, None);
        for item_200 in var_199 {
            #[allow(unused_mut)]
            let mut entry_202 = list_201.entry();
            entry_202.string(item_200);
        }
        list_201.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_set_load_balancer_policies_of_listener(
    input: &crate::input::SetLoadBalancerPoliciesOfListenerInput,
) -> Result<smithy_http::body::SdkBody, std::convert::Infallible> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        smithy_query::QueryWriter::new(&mut out, "SetLoadBalancerPoliciesOfListener", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_203 = writer.prefix("LoadBalancerName");
    if let Some(var_204) = &input.load_balancer_name {
        scope_203.string(var_204);
    }
    #[allow(unused_mut)]
    let mut scope_205 = writer.prefix("LoadBalancerPort");
    {
        scope_205.number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.load_balancer_port).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_206 = writer.prefix("PolicyNames");
    if let Some(var_207) = &input.policy_names {
        let mut list_209 = scope_206.start_list(false, None);
        for item_208 in var_207 {
            #[allow(unused_mut)]
            let mut entry_210 = list_209.entry();
            entry_210.string(item_208);
        }
        list_209.finish();
    }
    writer.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}