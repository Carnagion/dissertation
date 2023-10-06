#let todo(message: none) = raw(
    if message == none {
        "// TODO"
    } else {
        "// TODO: " + message
    },
    block: true,
    lang: "rust",
)

#let email(email) = link("mailto:" + email, raw(email))

#set text(font: "EB Garamond", size: 11pt)

#v(1fr)
#align(center)[
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

    // Integrated Runway Sequencing and De-Icing

    _Project Proposal_

    #v(0.1fr)

    #stack(dir: ltr, spacing: 1fr)[
        Indraneel Mahendrakumar\
        20372495\
        #email("psyim3@nottingham.ac.uk")\
    ][
        _Supervised By_\
        Geert De Maere\
        #email("geert.demaere@nottingham.ac.uk")\
    ]
]
#v(1fr)

// NOTE: Done after cover page since we don't want page numbers to show up on it
#set page(
    numbering: "1",
    number-align: center,
)

#set heading(numbering: "1.1")

#set par(justify: true)

#outline()
#pagebreak()

= Introduction

Airports have a limited number of runways, serving as a bottleneck to the number of aircraft that can take off or land at any given time frame while adhering to strict safety and operational requirements. The efficient scheduling of the take-off and landing of aircraft is thus critical for maximising the capacity of airports, and has a significant impact on operational costs, fuel emissions, and flight delays.

#todo(message: "Tweak first para, maybe mention previous approaches to runway sequencing")

De-icing further complicates this.

#todo(message: "Research more, expand on de-icing")

Such algorithms must be fast enough and reliable enough to be used in highly dynamic, real-time environments where failure is not an option.

#todo(message: "Add references")

= Objectives

#todo()

// 1. Problem definintion
// Mathematically model or define what's possible
// Look at Time Indexed Formulations - maybe extend some of the models proposed there
// Continuous formuation done by Beasley for aircraft landing - if A takes of at Ta and B takes off after A, then Ta + (sep between A and b) <= Tb
// Working with Beasley's formuation might be easier
// Furine
// Lieder

// 2. Investigate B&B
// Good starting point
// Integrate within a rolling window

// 3. Comparison with mathematical programming
// Extend Syntef's formulation

// 4. Vehicle Routing Problem
// Depending on size of aircraft being de-iced, the de-icing rakes may have to travel more and have insufficient fuel - small aircraft maybe 4, large maybe 3
// Stretch goal - may be a step too far

// 5. Visualisation
// What happens when?
// Think about KPIs - runway throughput, maximum delays, etc
// Huge improvement to understanding via visualisations - much better than raw data as numbers

// What is the contribution to science?
// - New problem, never been addressed before
// - New method
// - Fundamental insights (understanding the interaction between runway sequencing and de-icing - how much improvement can we get with integrating these solutions)
// - Using characteristics of the problem to simplify it

// 6. Post-processing optimisations
// De-icing sequence may be feasible given runway sequence (or the other way around)
// Then optimise de-icing itself without changing runway sequence and compromising its feasibility

// 7. Simulation
// Look at what happens over time instead of solving a single instance
// Could result in instability in sequences

= Project Plan

#todo(message: "Start with simple runway sequencing (branch & bound) and then move on to de-icing?")

= References

#todo()

// Partial integrated = Barry Smith, using some of the constraints from problem A to solve problem B
// Impose one constraint or characteristic manipulated in an earlier step to improve a later step
// Flight scheduling, fleet assignment, aircraft routing, crew scheduling
// FS = match capacity to demands (lot of demand for a route = put a big aircraft on it)
// FA = important for Crew Sched since crews can only work on certain aircraft
// Station purity = minimise number of aircraft types per hub/airport, fleet sched cost went up but crew connection times went down, good example of partial integration

// Mathematical Modelling
// Model Building in Mathematical Programming, 5e