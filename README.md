# **Rust_energy_project**


### This project is an energy data collect in Switzerland since 01.01.2014 from Opendata.swiss : [Energy data - opendata.swiss](https://eur02.safelinks.protection.outlook.com/?url=https%3A%2F%2Fopendata.swiss%2Fen%2Fdataset%2Fenergiedashboard-ch-stromproduktion-swissgrid&data=05%7C02%7Cthibald.buchet%40jobtrek.ch%7C7021b3a9d6bd4cd81e8908dc8a2c4792%7Cde08453512a9406cbd84d4bbcdb1a7b4%7C0%7C0%7C638537170101698018%7CUnknown%7CTWFpbGZsb3d8eyJWIjoiMC4wLjAwMDAiLCJQIjoiV2luMzIiLCJBTiI6Ik1haWwiLCJXVCI6Mn0%3D%7C0%7C%7C%7C&sdata=7gN1GP%2BvuVI0NpkSdCevzcZfuA5lxF8tMFkIpBjiA80%3D&reserved=0).

### I use these data and do differents things with them :


1. Calculate the total energy production of the entire file.
2. Calculate the production by type of energy production.
3. Calculate the production by years.
4. Calculate the production by months.
5. Calculate the type of energy that produced the most all years combined.
6. Calculate the year with the most production.

### To do this, i use differents functions like :

- .sum, which allows you to add different elements and return the result
- .into_grouping_map_by, wich groups elements by their key and, at the same time, fold each group using some aggregating operation
- .parse, which allowed me to transform a string into f64 for compare these elements.

### You could find help in the [Rust documentation](https://doc.rust-lang.org/book/).
