# People Data Analytics

This app is a learning project and attempt to create a data-centric model and Graphql API of employee skills, capabilities, certifications and work over time.

- [ ] Model people and their roles on teams
- [ ] Model people's skills and validate them based on their work
- [ ] Model how teams fit into an org hierarchy
- [ ] Model organizational capacity and work in progress
- [ ] Time-series modelling of changes to the organization over time as people change roles, learn and evolve.

It also includes :
- [x] User models
- [x] Automated Admin Generation
- [x] Authentication and sign-in
- [x] Email verification and reset password
- [x] Static files
- [x] Fluent integration for i18n

## Dependencies
* Diesel-cli

## Setup
* Clone the repository
* Create `.env` file with the following environmental variables:
    * COOKIE_SECRET_KEY=MINIMUM32CHARACTERS
    * DATABASE_URL
    * SENDGRID_API_KEY=YOUR_API_KEY
    * ADMIN_NAME="YOUR NAME"
    * ADMIN_EMAIL=your@email.com
    * ADMIN_PASSWORD=MINIMUM12CHARACTERS
    * ENVIRONMENT=test
* Change APP_NAME const in lib.rs to your app
* `diesel migration run`
* `cargo run`
