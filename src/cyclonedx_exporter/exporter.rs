





// #[derive(Serialize)]
// struct CycloneDxComponent {
//     supplier: String,
//     name: String,
//     version: String,
//     #[serde(rename = "type")]
//     component_type: String,
//     licenses: Vec<LicenseChoice>,
//     purl: String,
//     pedigree: String,
//     #[serde(rename = "bom-ref")]
//     bom_ref: String,
// }

// impl CycloneDxComponent {
//     fn from_series(row: &DataFrame, idx: usize) -> Self {
//         CycloneDxComponent {
//             supplier: row.column("supplier").unwrap().utf8().unwrap().get(idx).unwrap().to_string(),
//             name: row.column("name").unwrap().utf8().unwrap().get(idx).unwrap().to_string(),
//             version: row.column("version").unwrap().utf8().unwrap().get(idx).unwrap().to_string(),
//             component_type: row.column("type").unwrap().utf8().unwrap().get(idx).unwrap().to_string(),
//             licenses: vec![LicenseChoice {
//                 license: Some(License {
//                     id: Some(LicenseId {
//                         id: row.column("licenses").unwrap().utf8().unwrap().get(idx).unwrap().to_string(),
//                     }),
//                     ..Default::default()
//                 }),
//                 ..Default::default()
//             }],
//             purl: row.column("purl").unwrap().utf8().unwrap().get(idx).unwrap().to_string(),
//             pedigree: row.column("pedigree").unwrap().utf8().unwrap().get(idx).unwrap().to_string(),
//             bom_ref: format!(
//                 "{}_{}",
//                 row.column("name").unwrap().utf8().unwrap().get(idx).unwrap(),
//                 row.column("version").unwrap().utf8().unwrap().get(idx).unwrap()
//             ),
//         }
//     }
// }







// use cyclonedx_bom::{models::property::Properties, prelude::*};

 #[allow(unused_imports, unused_braces)]
use {
    std:: {
        fs::File,
        io::Write,
        error::Error
    }
};


 #[allow(unused_imports, unused_braces)]
use {
    cyclonedx_bom::{
        models:: {
            bom::{
                Bom, UrnUuid
            },
            metadata::Metadata,
            property::{
                Properties, Property
            },
            organization::OrganizationalContact,
            component::{
                Component, Pedigree, Classification, Components
            },
            license::{
                License, LicenseChoice, Licenses
            },
            vulnerability::{
                Vulnerabilities, Vulnerability
            },
            vulnerability_source::VulnerabilitySource,
            vulnerability_rating::{
                VulnerabilityRatings, VulnerabilityRating, Score, Severity, ScoreMethod
            },
            vulnerability_target::{
                VulnerabilityTargets, VulnerabilityTarget
            }
        },
        external_models:: {
            date_time::DateTime,
            uri:: {
                Purl, UriError
            },
            normalized_string::NormalizedString
        },
    }
};



fn main() -> Result<(), Box<dyn Error>> {

    // * Properties in Header 
    // TODO: Get Property values from Database via Dataframe  
    let properties = vec![
        Property::new("sourcingNr", "AU2018003773"),
        Property::new("vorgangsbezeichnung", "Beauftragung XYZ"),
        Property::new("vehicleproject", "AU123")
    ];

    // * Authors in Header
    // TODO: Get Property values from Database via Dataframe  
    let authors = vec![
        OrganizationalContact::new("Accenture GmbH", Some("psirt@accenture.com"))
    ];

   
    // * Component Block
    // TODO: Get Property values from Database via Dataframe  
    let component_bom_ref = UrnUuid::generate().to_string().replace("urn:uuid:", "");
    let ref_vul = component_bom_ref.clone();
    let target_ref = component_bom_ref.clone();

    let license = License::named_license("MIT");
    let license_choice = LicenseChoice::License(license);

    let mut component = Component::new(
        Classification::Library, 
        "ahash",
        "0.7.6",
        Some(component_bom_ref)
    );
    component.licenses = Some(Licenses(vec![license_choice]));
    component.purl = match Purl::new("cargo", &*component.name, component.version.as_ref().unwrap()) {
        Ok(purl) => Some(purl),
        Err(e) => {
            eprintln!("Error creating Purl: {}", e);
            None
        },
    };
    component.pedigree = Some(Pedigree {
        notes: Some("MIT License".to_string()),
        ancestors: None,
        descendants: None,
        variants: None,
        commits: None,
        patches: None
    });

    // * Vulnerability Block
    // TODO: Get Property values from Database via Dataframe 

    let source = VulnerabilitySource {
        name: Some(NormalizedString::new("NVD")),
        url: None
    };

    let mut rating = VulnerabilityRating::new(
        Score::from_f32(7.2), 
        Some(Severity::High), 
        Some(ScoreMethod::CVSSv3)
    );
    rating.vulnerability_source = Some(source.clone());

    let target = VulnerabilityTarget::new(target_ref); 


    let mut vulnerability = Vulnerability::new(Some(ref_vul));
    vulnerability.id = Some(NormalizedString::new("CVE-2014-0069"));
    vulnerability.vulnerability_source = Some(source);
    vulnerability.vulnerability_ratings = Some(VulnerabilityRatings(vec![rating]));
    vulnerability.description = Some("Need to check , how to fill this attribute valid text".to_string());
    vulnerability.vulnerability_targets = Some(VulnerabilityTargets(vec![target]));


    // * Bom builder.
    let jbom = Bom {
        serial_number: Some(
            UrnUuid::generate()
        ),
        version: 1,
        metadata: Some(Metadata {
            timestamp: Some(DateTime::now().expect("Timestamp failed")),
            properties: Some(Properties(properties)),
            authors: Some(authors),
            ..Default::default()
        }),
        components: Some(Components(vec![component])),
        vulnerabilities: Some(Vulnerabilities(vec![vulnerability])),
        ..Default::default()
    };

    let mut output = Vec::<u8>::new();

    jbom.output_as_json_v1_4(&mut output).expect("failed to write sbom");
    

    // Write the JSON to a file
    let mut file = File::create("output.json")?;
    file.write_all(&output)?;

    println!("BOM written to output.json");

    Ok(())

}
    

