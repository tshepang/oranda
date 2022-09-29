use super::options::Options;

pub fn make_head(options: &Options) -> String {
    format!(
        r#"
   <!DOCTYPE html>
   <html lang="en">
   <head>
   
   <meta charset="utf-8">
   <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1" />
   <meta name="viewport" content="width=device-width, initial-scale=1.0">
   <link rel="stylesheet" href="styles.css"></link>
   
   <title>{name}</title>
   <meta name="description" content={description} />
   </head>
   <body>
   <div id="oranda"><div class="body"><div class="container">
   "#,
        name = options.name.as_ref().unwrap(),
        description = options.description.as_ref().unwrap()
    )
}
