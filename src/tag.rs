pub struct Tag {
    pub (self) name: String,
    pub (self) color: [String; 3]
}

impl Tag
{

    // constructor
    pub fn new(name: &str, color: [String; 3]) -> Tag
    {
        return Tag {
            name: String::from(name),
            color
        }
    }

}