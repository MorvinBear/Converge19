use hdk::{
    entry_definition::ValidatingEntryType,
    entry_definition::ValidatingLinkDefinition,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Convergence {
    name: String,
    description: String
}

pub fn create(entry: Convergence) -> ZomeApiResult<Address> {
    let entry = Entry::App("convergence".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn get(address: Address) -> ZomeApiResult<Convergence> {
    hdk::utils::get_as_type(address)
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "convergence",
        description: "A convergence of a possibility",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Convergence>| {
            Ok(())
        },

        links: [ link_to_possibility() ]
    )
}

pub fn link_to_possibility() -> ValidatingLinkDefinition {
    link!(
       direction: hdk::LinkDirection::To,
       other_type: "possibility",
       link_type: "generatedFrom",
       validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
       },
        validation: | _validation_data: hdk::LinkValidationData | {
            Ok(())
        }
    )
}
