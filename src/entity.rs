#[derive(Serialize, Deserialize)]
pub struct ZipResult {
    #[serde(rename="post code")]
    post_code: String,
    country: String,
    #[serde(rename="country abbreviation")]
    country_abbr: String,
    places: Vec<Place>,
}

#[derive(Serialize, Deserialize)]
pub struct Place {
    #[serde(rename="place name")]
    place_name: String,
    latitude: String,
    longitude: String,
    state: String,
    #[serde(rename="state abbreviation")]
    state_abbr: String,
}

impl ::std::fmt::Display for ZipResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self.places.iter().nth(0) {
            Some(place) => write!(f, "{}, {} {}", place.place_name, place.state, self.post_code),
            None => f.write_str("No matches found"),
        }
    }
}
