// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_start_human_loop_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartHumanLoopInput,
) {
    if let Some(var_1) = &input.data_attributes {
        let mut object_2 = object.key("DataAttributes").start_object();
        crate::json_ser::serialize_structure_human_loop_data_attributes(&mut object_2, var_1);
        object_2.finish();
    }
    if let Some(var_3) = &input.flow_definition_arn {
        object.key("FlowDefinitionArn").string(var_3);
    }
    if let Some(var_4) = &input.human_loop_input {
        let mut object_5 = object.key("HumanLoopInput").start_object();
        crate::json_ser::serialize_structure_human_loop_input(&mut object_5, var_4);
        object_5.finish();
    }
    if let Some(var_6) = &input.human_loop_name {
        object.key("HumanLoopName").string(var_6);
    }
}

pub fn serialize_structure_stop_human_loop_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopHumanLoopInput,
) {
    if let Some(var_7) = &input.human_loop_name {
        object.key("HumanLoopName").string(var_7);
    }
}

pub fn serialize_structure_human_loop_data_attributes(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HumanLoopDataAttributes,
) {
    if let Some(var_8) = &input.content_classifiers {
        let mut array_9 = object.key("ContentClassifiers").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
}

pub fn serialize_structure_human_loop_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HumanLoopInput,
) {
    if let Some(var_11) = &input.input_content {
        object.key("InputContent").string(var_11);
    }
}
