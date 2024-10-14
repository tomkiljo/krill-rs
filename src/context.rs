use fake::{
    faker::{
        address::raw::{
            BuildingNumber, CityName, CityPrefix, CitySuffix, CountryCode, CountryName, Geohash,
            Latitude, Longitude, PostCode, SecondaryAddress, SecondaryAddressType, StateAbbr,
            StateName, StreetName, StreetSuffix, TimeZone, ZipCode,
        },
        boolean::raw::Boolean,
        color::raw::{HexColor, HslColor, HslaColor, RgbColor, RgbaColor},
        company::raw::{
            Bs, BsAdj, BsNoun, BsVerb, Buzzword, BuzzwordMiddle, BuzzwordTail, CatchPhrase,
            CompanyName, CompanySuffix, Industry, Profession,
        },
        creditcard::raw::CreditCardNumber,
        currency::raw::{CurrencyCode, CurrencyName, CurrencySymbol},
        filesystem::raw::{
            DirPath, FileExtension, FileName, FilePath, MimeType, Semver, SemverStable,
            SemverUnstable,
        },
        finance::raw::{Bic, Isin},
        internet::raw::{
            DomainSuffix, IPv4, IPv6, MACAddress, Password, SafeEmail, UserAgent, Username,
        },
        job::raw::{
            Field as JobField, Position as JobPosition, Seniority as JobSeniority,
            Title as JobTitle,
        },
        lorem::raw::{Paragraphs, Sentences, Words},
        name::raw::{
            FirstName, LastName, Name as FullName, Suffix as NameSuffix, Title as NamePrefix,
        },
        number::raw::NumberWithFormat,
    },
    locales,
    uuid::{UUIDv1, UUIDv3, UUIDv4, UUIDv5},
    Fake,
};
use handlebars::{handlebars_helper, Handlebars};
use std::{cmp, collections::BTreeMap, env, str::FromStr};

use crate::KeyValue;

pub struct Context<'ctx> {
    data: BTreeMap<String, String>,
    registry: Handlebars<'ctx>,
}

impl<'ctx> Context<'ctx> {
    pub fn _new() -> Self {
        let data = BTreeMap::new();
        let registry = Self::new_registry();
        Context { data, registry }
    }

    pub fn from_args(params: Vec<KeyValue>) -> Self {
        let data: BTreeMap<String, String> = params
            .into_iter()
            .map(|KeyValue(key, value)| (key, value))
            .collect();
        let registry = Self::new_registry();
        Context { data, registry }
    }

    fn new_registry() -> Handlebars<'ctx> {
        let mut registry = Handlebars::new();
        registry.register_escape_fn(handlebars::no_escape);
        registry.set_strict_mode(true);
        registry.register_helper("$env", Box::new(env_helper));
        registry.register_helper("$systemArch", Box::new(arch_helper));
        registry.register_helper("$systemOS", Box::new(os_helper));
        registry.register_helper("$systemFamily", Box::new(family_helper));
        registry.register_helper("$timestamp", Box::new(timestamp_helper));

        registry.register_helper("$randomInt", Box::new(random_int));
        registry.register_helper("$randomNumber", Box::new(random_number));
        registry.register_helper("$randomBoolean", Box::new(random_boolean));

        registry.register_helper("$randomGUID", Box::new(random_guid));
        registry.register_helper("$randomUUIDv1", Box::new(random_uuidv1));
        registry.register_helper("$randomUUIDv3", Box::new(random_uuidv3));
        registry.register_helper("$randomUUIDv4", Box::new(random_uuidv4));
        registry.register_helper("$randomUUIDv5", Box::new(random_uuidv5));

        registry.register_helper("$randomHexColor", Box::new(random_hex_color));
        registry.register_helper("$randomHSLColor", Box::new(random_hsl_color));
        registry.register_helper("$randomHSLAColor", Box::new(random_hsla_color));
        registry.register_helper("$randomRGBColor", Box::new(random_rgb_color));
        registry.register_helper("$randomRGBAColor", Box::new(random_rgba_color));

        registry.register_helper("$randomWord", Box::new(random_word));
        registry.register_helper("$randomSentence", Box::new(random_sentence));
        registry.register_helper("$randomParagraph", Box::new(random_paragraph));

        registry.register_helper("$randomFullName", Box::new(random_full_name));
        registry.register_helper("$randomFirstName", Box::new(random_first_name));
        registry.register_helper("$randomLastName", Box::new(random_last_name));
        registry.register_helper("$randomNamePrefix", Box::new(random_name_prefix));
        registry.register_helper("$randomNameSuffix", Box::new(random_name_suffix));

        registry.register_helper("$randomJobSeniority", Box::new(random_job_seniority));
        registry.register_helper("$randomJobField", Box::new(random_job_field));
        registry.register_helper("$randomJobPosition", Box::new(random_job_position));
        registry.register_helper("$randomJobTitle", Box::new(random_job_title));

        registry.register_helper("$randomCompanySuffix", Box::new(random_company_suffix));
        registry.register_helper("$randomCompanyName", Box::new(random_company_name));
        registry.register_helper("$randomBuzzword", Box::new(random_buzzword));
        registry.register_helper("$randomBuzzwordMiddle", Box::new(random_buzzword_middle));
        registry.register_helper("$randomBuzzwordTail", Box::new(random_buzzword_tail));
        registry.register_helper("$randomCatchPhrase", Box::new(random_catchphrase));
        registry.register_helper("$randomBSVerb", Box::new(random_bs_verb));
        registry.register_helper("$randomBSAdj", Box::new(random_bs_adj));
        registry.register_helper("$randomBSNoun", Box::new(random_bs_noun));
        registry.register_helper("$randomBS", Box::new(random_bs));
        registry.register_helper("$randomProfession", Box::new(random_profession));
        registry.register_helper("$randomIndustry", Box::new(random_industry));

        registry.register_helper("$randomCityPrefix", Box::new(random_city_prefix));
        registry.register_helper("$randomCitySuffix", Box::new(random_city_suffix));
        registry.register_helper("$randomCityName", Box::new(random_city_name));
        registry.register_helper("$randomCountryName", Box::new(random_country_name));
        registry.register_helper("$randomCountryCode", Box::new(random_country_code));
        registry.register_helper("$randomStreetSuffix", Box::new(random_street_suffix));
        registry.register_helper("$randomStreetName", Box::new(random_street_name));
        registry.register_helper("$randomTimeZone", Box::new(random_time_zone));
        registry.register_helper("$randomStateName", Box::new(random_state_name));
        registry.register_helper("$randomStateAbbr", Box::new(random_state_abbr));
        registry.register_helper(
            "$randomSecondaryAddressType",
            Box::new(random_secondary_address_type),
        );
        registry.register_helper(
            "$randomSecondaryAddress",
            Box::new(random_secondary_address),
        );
        registry.register_helper("$randomZipCode", Box::new(random_zip_code));
        registry.register_helper("$randomPostCode", Box::new(random_post_code));
        registry.register_helper("$randomBuildingNumber", Box::new(random_building_number));
        registry.register_helper("$randomLatitude", Box::new(random_latitude));
        registry.register_helper("$randomLongitude", Box::new(random_longitude));
        registry.register_helper("$randomGeohash", Box::new(random_geohash));

        registry.register_helper("$randomFilePath", Box::new(random_file_path));
        registry.register_helper("$randomFileName", Box::new(random_file_name));
        registry.register_helper("$randomFileExtension", Box::new(random_file_extension));
        registry.register_helper("$randomDirPath", Box::new(random_dir_path));
        registry.register_helper("$randomMimeType", Box::new(random_mime_type));
        registry.register_helper("$randomSemver", Box::new(random_semver));
        registry.register_helper("$randomSemverStable", Box::new(random_semver_stable));
        registry.register_helper("$randomSemverUnstable", Box::new(random_semver_unstable));

        registry.register_helper("$randomDomainSuffix", Box::new(random_domain_suffix));
        registry.register_helper("$randomSafeEmail", Box::new(random_safe_email));
        registry.register_helper("$randomUsername", Box::new(random_username));
        registry.register_helper("$randomPassword", Box::new(random_password));
        registry.register_helper("$randomIPv4", Box::new(random_ipv4));
        registry.register_helper("$randomIPv6", Box::new(random_ipv6));
        registry.register_helper("$randomMACAddress", Box::new(random_mac_address));
        registry.register_helper("$randomUserAgent", Box::new(random_user_agent));

        registry.register_helper("$randomCurrencyCode", Box::new(random_currency_code));
        registry.register_helper("$randomCurrencyName", Box::new(random_currency_name));
        registry.register_helper("$randomCurrencySymbol", Box::new(random_currency_symbol));

        registry.register_helper(
            "$randomCreditCardNumber",
            Box::new(random_credit_card_number),
        );
        registry.register_helper("$randomBIC", Box::new(random_bic));
        registry.register_helper("$randomISIN", Box::new(random_isin));

        registry
    }

    pub fn render(&self, template: &str) -> anyhow::Result<String> {
        self.registry
            .render_template(template, &self.data)
            .map_err(|e| e.into())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn variable(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}

// Helper function to render environment variables
handlebars_helper!(env_helper: |param: str| env::var(param).unwrap_or_default());

// Helper functions to render system data
handlebars_helper!(arch_helper: |*_args| std::env::consts::ARCH.to_string());
handlebars_helper!(os_helper: |*_args| std::env::consts::OS.to_string());
handlebars_helper!(family_helper: |*_args| std::env::consts::FAMILY.to_string());
handlebars_helper!(timestamp_helper: |{format: str = "unix", offset: str = "utc"}| {
    let time = match *&offset {
        "utc" => chrono::Utc::now().into(),
        "local" => chrono::Local::now().into(),
        _ => {
            let tz = chrono::offset::FixedOffset::from_str(offset).unwrap();
            chrono::Utc::now().with_timezone(&tz)
        },
    };
    match *&format {
        "unix" => time.timestamp().to_string(),
        "unix_millis" => time.timestamp_millis().to_string(),
        "unix_micros" => time.timestamp_micros().to_string(),
        "rfc2822" => time.to_rfc2822(),
        "iso8601" | "rfc3339" => time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "iso8601_millis" | "rfc3339_millis" => time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        "iso8601_nanos" | "rfc3339_nanos" => time.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
        _ => time.format(format).to_string(),
    }
});

// Helper functions to render random data
handlebars_helper!(random_int: |{min: i32 = 0, max: i32 = 1000}| (min..=max).fake::<i32>());
handlebars_helper!(random_number: |{format: str = "#.#"}| NumberWithFormat(locales::EN, &format).fake::<String>());
handlebars_helper!(random_boolean: |{ratio: u8 = 50}| Boolean(locales::EN, ratio).fake::<bool>());

handlebars_helper!(random_guid: |*_args| UUIDv4.fake::<String>());
handlebars_helper!(random_uuidv1: |*_args| UUIDv1.fake::<String>());
handlebars_helper!(random_uuidv3: |*_args| UUIDv3.fake::<String>());
handlebars_helper!(random_uuidv4: |*_args| UUIDv4.fake::<String>());
handlebars_helper!(random_uuidv5: |*_args| UUIDv5.fake::<String>());

handlebars_helper!(random_hex_color: |*_args| HexColor(locales::EN).fake::<String>());
handlebars_helper!(random_hsl_color: |*_args| HslColor(locales::EN).fake::<String>());
handlebars_helper!(random_hsla_color: |*_args| HslaColor(locales::EN).fake::<String>());
handlebars_helper!(random_rgb_color: |*_args| RgbColor(locales::EN).fake::<String>());
handlebars_helper!(random_rgba_color: |*_args| RgbaColor(locales::EN).fake::<String>());

handlebars_helper!(random_word: |{count: usize = 1, min: usize = 0, max: usize = 0, sep: str = " "}|
    Words(locales::EN, cmp::max(count, min)..cmp::max(count, max)+1).fake::<Vec<String>>().join(&sep));
handlebars_helper!(random_sentence: |{count: usize = 1, min: usize = 0, max: usize = 0, sep: str = " "}|
    Sentences(locales::EN, cmp::max(count, min)..cmp::max(count, max)+1).fake::<Vec<String>>().join(&sep));
handlebars_helper!(random_paragraph: |{count: usize = 1, min: usize = 0, max: usize = 0, sep: str = "\n"}|
    Paragraphs(locales::EN, cmp::max(count, min)..cmp::max(count, max)+1).fake::<Vec<String>>()
        .into_iter().map(|p| p.replace("\n", &sep)).collect::<Vec<String>>().join(&sep));

handlebars_helper!(random_full_name: |*_args| FullName(locales::EN).fake::<String>());
handlebars_helper!(random_first_name: |*_args| FirstName(locales::EN).fake::<String>());
handlebars_helper!(random_last_name: |*_args| LastName(locales::EN).fake::<String>());
handlebars_helper!(random_name_prefix: |*_args| NamePrefix(locales::EN).fake::<String>());
handlebars_helper!(random_name_suffix: |*_args| NameSuffix(locales::EN).fake::<String>());

handlebars_helper!(random_job_seniority: |*_args| JobSeniority(locales::EN).fake::<String>());
handlebars_helper!(random_job_field: |*_args| JobField(locales::EN).fake::<String>());
handlebars_helper!(random_job_position: |*_args| JobPosition(locales::EN).fake::<String>());
handlebars_helper!(random_job_title: |*_args| JobTitle(locales::EN).fake::<String>());

handlebars_helper!(random_company_suffix: |*_args| CompanySuffix(locales::EN).fake::<String>());
handlebars_helper!(random_company_name: |*_args| CompanyName(locales::EN).fake::<String>());
handlebars_helper!(random_buzzword: |*_args| Buzzword(locales::EN).fake::<String>());
handlebars_helper!(random_buzzword_middle: |*_args| BuzzwordMiddle(locales::EN).fake::<String>());
handlebars_helper!(random_buzzword_tail: |*_args| BuzzwordTail(locales::EN).fake::<String>());
handlebars_helper!(random_catchphrase: |*_args| CatchPhrase(locales::EN).fake::<String>());
handlebars_helper!(random_bs_verb: |*_args| BsVerb(locales::EN).fake::<String>());
handlebars_helper!(random_bs_adj: |*_args| BsAdj(locales::EN).fake::<String>());
handlebars_helper!(random_bs_noun: |*_args| BsNoun(locales::EN).fake::<String>());
handlebars_helper!(random_bs: |*_args| Bs(locales::EN).fake::<String>());
handlebars_helper!(random_profession: |*_args| Profession(locales::EN).fake::<String>());
handlebars_helper!(random_industry: |*_args| Industry(locales::EN).fake::<String>());

handlebars_helper!(random_city_prefix: |*_args| CityPrefix(locales::EN).fake::<String>());
handlebars_helper!(random_city_suffix: |*_args| CitySuffix(locales::EN).fake::<String>());
handlebars_helper!(random_city_name: |*_args| CityName(locales::EN).fake::<String>());
handlebars_helper!(random_country_name: |*_args| CountryName(locales::EN).fake::<String>());
handlebars_helper!(random_country_code: |*_args| CountryCode(locales::EN).fake::<String>());
handlebars_helper!(random_street_suffix: |*_args| StreetSuffix(locales::EN).fake::<String>());
handlebars_helper!(random_street_name: |*_args| StreetName(locales::EN).fake::<String>());
handlebars_helper!(random_time_zone: |*_args| TimeZone(locales::EN).fake::<String>());
handlebars_helper!(random_state_name: |*_args| StateName(locales::EN).fake::<String>());
handlebars_helper!(random_state_abbr: |*_args| StateAbbr(locales::EN).fake::<String>());
handlebars_helper!(random_secondary_address_type: |*_args| SecondaryAddressType(locales::EN).fake::<String>());
handlebars_helper!(random_secondary_address: |*_args| SecondaryAddress(locales::EN).fake::<String>());
handlebars_helper!(random_zip_code: |*_args| ZipCode(locales::EN).fake::<String>());
handlebars_helper!(random_post_code: |*_args| PostCode(locales::EN).fake::<String>());
handlebars_helper!(random_building_number: |*_args| BuildingNumber(locales::EN).fake::<String>());
handlebars_helper!(random_latitude: |*_args| Latitude(locales::EN).fake::<String>());
handlebars_helper!(random_longitude: |*_args| Longitude(locales::EN).fake::<String>());
handlebars_helper!(random_geohash: |{precision: u8 = 12}| Geohash(locales::EN, precision).fake::<String>());

handlebars_helper!(random_file_path: |*_args| FilePath(locales::EN).fake::<String>());
handlebars_helper!(random_file_name: |*_args| FileName(locales::EN).fake::<String>());
handlebars_helper!(random_file_extension: |*_args| FileExtension(locales::EN).fake::<String>());
handlebars_helper!(random_dir_path: |*_args| DirPath(locales::EN).fake::<String>());
handlebars_helper!(random_mime_type: |*_args| MimeType(locales::EN).fake::<String>());
handlebars_helper!(random_semver: |*_args| Semver(locales::EN).fake::<String>());
handlebars_helper!(random_semver_stable: |*_args| SemverStable(locales::EN).fake::<String>());
handlebars_helper!(random_semver_unstable: |*_args| SemverUnstable(locales::EN).fake::<String>());

handlebars_helper!(random_domain_suffix: |*_args| DomainSuffix(locales::EN).fake::<String>());
handlebars_helper!(random_safe_email: |*_args| SafeEmail(locales::EN).fake::<String>());
handlebars_helper!(random_username: |*_args| Username(locales::EN).fake::<String>());
handlebars_helper!(random_password: |{min: usize = 16, max: usize = 32}| Password(locales::EN, min..max+1).fake::<String>());
handlebars_helper!(random_ipv4: |*_args| IPv4(locales::EN).fake::<String>());
handlebars_helper!(random_ipv6: |*_args| IPv6(locales::EN).fake::<String>());
handlebars_helper!(random_mac_address: |*_args| MACAddress(locales::EN).fake::<String>());
handlebars_helper!(random_user_agent: |*_args| UserAgent(locales::EN).fake::<String>());

handlebars_helper!(random_currency_code: |*_args| CurrencyCode(locales::EN).fake::<String>());
handlebars_helper!(random_currency_name: |*_args| CurrencyName(locales::EN).fake::<String>());
handlebars_helper!(random_currency_symbol: |*_args| CurrencySymbol(locales::EN).fake::<String>());

handlebars_helper!(random_credit_card_number: |*_args| CreditCardNumber(locales::EN).fake::<String>());
handlebars_helper!(random_bic: |*_args| Bic(locales::EN).fake::<String>());
handlebars_helper!(random_isin: |*_args| Isin(locales::EN).fake::<String>());
