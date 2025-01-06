# Vis

![vis](vis-icon.jpg)

_It is Dutch for fish_

### NB I just started, needs a lot of work

sample vis file:

```
markup {
  lanes {
    functions {
      calc: "Calculation"
      acc_interest_calc: "Account interest Calculation"
      interest_rates: "Interest Rates"
      config: "Configuration"
      nob: "NoB" {
        nob_execution: "NoB Execution"
        coll_reinst_inst: "Collection of Reinstatement instructions"
      }
      reporting: "Reporting"
    }
    systems {
      bank: "Bank" {
        bank_motor: "Bank Motor"
        bank_scripts: "Bank Scripts"
        bank_client: "Bank Client"
        bank_db: "Bank DB"
      }
      interest_engine: "InterestEngine"
    }
  }
  bank-->calc
  bank_scripts--<>bank_db
  bank_motor--<>bank_db
  interest_engine-->calc
}

styles {
  lanes(group) {
    type: textnode
    orientation: horizontal
    shape: rectangle
    font-family: arial
    border-width: 1px
    border-color: gray
  }
  functions(group) {
    background-color: yellow
    font-family: arial
    border-radius: 20px
    border-width: 1px
    border-color: gray
  }
  systems(group) {
    background-color: lightblue
  }
  tag1: "⚭" { // how will this work?
    right:0px
    top:0px
  }
  tag2: {
    itchy: scratchy
  }
}
```

Will have to be turned into an architecture diagram... we'll see how it goes!
