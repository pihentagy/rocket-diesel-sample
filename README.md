# rocket-diesel-sample
Simple program to demonstrate rocket 0.4 and diesel (not working!)

```
error[E0277]: the trait bound `&models::Workload: diesel::Insertable<schema::workloads::table>` is not satisfied
  --> src/main.rs:40:43
   |
40 |     diesel::insert_into(workloads).values(&res).execute(&*conn);
   |                                           ^^^^ the trait `diesel::Insertable<schema::workloads::table>` is not implemented for `&models::Workload`
```

If I add Insertable to Workload, I get

```
error[E0433]: failed to resolve: use of undeclared type or module `workloads`
 --> src/models.rs:7:12
  |
7 | pub struct Workload {
  |            ^^^^^^^^ use of undeclared type or module `workloads`
  ```
