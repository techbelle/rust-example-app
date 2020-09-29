# rust-example-app
This micro application can be used by beginners to explore Rust coding on their own alongside a free MongoDB Atlas database. Note: but seriously, this is a beginner app :)

# Getting Started
1. You'll want to follow the rust installation instructions at https://www.rust-lang.org/tools/install
2. Create an Atlas cluster at cloud.mongodb.com - in my example, I'm using a free tier cluster and I've called it `rustdemo`
3. Create a database user via the Atlas UI under the  Database Access nav. Make sure you give this read/write access.
4. Allow your IP address access to the new database via the Network Access nav.
5. Clone the rust-example-app repository onto your laptop


# Initializing the Project
1. After you've created your demo Atlas cluster, you'll want to copy your connection string to your new database. In my example, I set the database as an environmental variable via the terminal, like so:
`Rust/rust-example-app$ MONGODB_URI='mongodb+srv://user:PASSWORD@rustdemo-xg44p.mongodb.net/test?retryWrites=true&w=majority' cargo run`
2. Access localhost on your machine and the app will be running on localhost port 8080. Submit some text into the form. 
3. Navigate back to your Atlas cluster via cloud.mongodb.com and click the "Collections" button. This will take you to the data explorer, where you'll be able to view the documents as they are created. We can see that each entry is its own document, with this format:
`{"_id":{"$oid":"5eb1784200fb14190039c0ef"},"keyword":"cat"}`






