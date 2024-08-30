# Fermyon Todo

This application is a FullStack Serverless Rust application containing the following elements:

* Leptos Frontend embedded in a static files Spin component
* API Backend build on Spin Rust SDK
* SQLITE Database as data storage

You need to apply the correct URL in the Frontend Service (todo_service.rs) to link to the Fermyon API when deployed, shown during the deployment:

````
Waiting for application to become ready................ ready

View application:   https://todo-2uxjwrwy.fermyon.app/
  Routes:
  - ui: https://todo-XXXXXXXXXX.fermyon.app/ui (wildcard)
  - api: https://todo-XXXXXXXXXX.fermyon.app/api (wildcard)
````

Some Fermyon commands:

---

Login to Fermyon Cloud

````shell
spin login
````

Local migration of DB

````shell
spin up --sqlite @migration.sql
````

For local build and running

````shell
spin build --up
````

Live refresh during development

````shell
spin watch
````

Deploying to the cloud

```shell
spin deploy
```

Cloud migration of DB (replace the name of DB)

````shell
spin cloud sqlite execute -d lively-grape @migration.sql 
````