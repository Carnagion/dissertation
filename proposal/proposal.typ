#let email(email) = link("mailto:" + email, raw(email))

#set text(font: "EB Garamond", size: 11pt)

#v(1fr)
#align(center)[
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

    _Project Proposal_

    #v(1.3em)

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
#set page(numbering: "1")

#set heading(numbering: "1.1")
#show heading: set block(above: 2em, below: 1.3em)

#set par(justify: true)

#outline()
#pagebreak()

= Introduction

Although it is possible to construct additional runways or airports, it may not always be feasible due to the high infrastructure and planning costs and the (lack of) availability of land. Therefore, efficient scheduling of runway operations is crucial in maximising the capacity of existing runways and airports.

This requires careful consideration of multiple factors including the type of operation (i.e. taking off or landing), aircraft class of the preceding and succeeding operation, the allocated time frame for the operation, and the number of available runways during that time. When an aircraft takes off or lands, it produces air turbulence that affects the following aircraft. The impact of this turbulence depends on the aircraft's class, which is based on its size and weight. // FROM: Lieder, dynamic programming
Aircraft are also assigned a specific window of time for taking off or landing, based on surrounding air traffic. // FROM: Lieder, scheduling aircraft
Furthermore, each crew member may also only be trained to operate certain kinds of aircraft, and may be forced to wait for long periods of time after completing a flight if another aircraft of the same kind is not yet available for them. // TODO: Source?
Runway sequences must therefore meet strict separation requirements that depend on the aforementioned factors, while minimising operational costs, fuel emissions, flight delays, and crew wait time. // FROM: Lieder, scheduling aircraft

However, de-icing presents an additional challenge to this.

// TODO: Talk about crew and fleet scheduling

// TODO: Talk about fundamental insights

// TODO: Reconsider use of the word "optimal"?

= Objectives

The primary aim of this project is to investigate the integrated runway sequencing and de-icing problem and develop an algorithm capable of solving it. This will provide a deeper insight into the problem's fundamental characteristics and the interactions between runway sequencing and de-icing, as well as the potential benefits of integrating their solutions.

// TODO: Should insights be mentioned above?

The key objectives are thus as follows:

1. Investigate previous approaches to runway sequencing. The mathematical models and formulations proposed in prior research may not be directly applicable to this project, as there have been few efforts until now to tackle runway sequencing and de-icing in an integrated fashion. Thus, there will be a need to understand and then adapt or extend these models so they are suitable for the integrated problem.

// TODO: Maybe find a way to avoid saying "runway sequencing and de-icing" a dozen times

2. Design and implement an algorithm that provides optimal solutions for the integrated runway sequencing and de-icing problem. The algorithm must be generic enough to work with data from different sources (i.e. different airports and datasets), by using a set of common features and characteristics in the data. Additionally, it must be fast and reliable enough to be viable in highly dynamic, real-time situations where unexpected failure is not an option.

3. Evaluate the performance of the algorithm. This will involve benchmarking it on known and available datasets, and comparing it to existing solutions. A simulation that is more representative of real-world data and use cases will also be used to run the algorithm on multiple problem instances over a longer period of time. This will help expose any issues, such as instability in the generated sequences, that may not be visible in individual runs.

4. Develop a tool for visualising the outputs and possibly intermediate data produced by the algorithm. This will provide a more intuitive, human-friendly view intended to aid users' understanding.

= Plan

The overall work plan is to start off by investigating previous approaches to the problem and establishing a mathematical model, as any further work will be reliant on this. Development will initially consist of implementing a simple branch-and-bound algorithm that follows the model, and later extending it with a rolling window. De-icing will then be integrated into the algorithm after the interim report deadline. Finally, the algorithm will be evaluated and simulated, and the visualisation tool will be developed before writing the final dissertation.

The following Gantt chart outlines the plan along with timelines:

/ A: Write the project prposal
/ B: Research previous approaches into runway sequencing and de-icing
/ C: Implement a naive branch-and-bound algorithm for runway sequencing
/ D: Extend the algorithm with a rolling window
/ E: Write the interim report
/ F: Christmas break
/ G: Prepare for exams
/ H: Integrate de-icing into the algorithm
/ I: Evaluate the performance of the algorithm and run simulations
/ J: Implement the visualisation tool
/ K: Write the final dissertation
/ L: Easter break

#import "@preview/timeliney:0.0.1": *

#timeline(show-grid: true, {
    headerline(group(([*2023*], 3)), group(([*2024*], 4)))
    let months = ("Oct", "Nov", "Dec", "Jan", "Feb", "Mar", "Apr")
    headerline(..months.map(group))

    let task-line-style = (stroke: 3pt + gray)

    // 1 = approx 30 days
    // 0.1 = approx 3 days

    taskgroup({
        // Write project proposal
        task("A", (0, 0.86), style: task-line-style)

        // Investigate previous approaches
        task("B", (0, 1.23), style: task-line-style)

        // Implement branch-and-bound
        task("C", (0.7, 1.43), style: task-line-style)

        // Implement rolling window
        task("D", (1.43, 2.13), style: task-line-style)

        // Write interim report
        task("E", (1.43, 2.36), style: task-line-style)

        // Christmas break
        task("F", (2.5, 3.5), style: task-line-style)

        // Exam preparation
        task("G", (2.9, 3.9), style: task-line-style)

        // Integrate de-icing
        task("H", (3.9, 4.6), style: task-line-style)

        // Evaluate performance
        task("I", (4.6, 5.1), style: task-line-style)

        // Implement visualiser
        task("J", (4.9, 5.5), style: task-line-style)

        // Write final dissertation
        task("K", (5.1, 6.63), style: task-line-style)

        // Easter break
        task("L", (5.93, 6.96), style: task-line-style)
    })

    let milestone-line-style = (stroke: (dash: "dashed"))

    milestone(
        at: 0.86,
        style: milestone-line-style,
        align(center)[
            *Project Proposal*\
            26 Oct
        ],
    )

    milestone(
        at: 2.36,
        style: milestone-line-style,
        align(center)[
            *Interim Report*\
            8 Dec
        ],
    )

    milestone(
        at: 6.63,
        style: milestone-line-style,
        align(center)[
            *Final Dissertation*\
            19 Apr
        ],
    )
})

= References

// TODO: Add citations where needed

// Notes

// 6-10-2023

// 1. Problem definintion
// Mathematically model or define what's possible
// Look at Time Indexed Formulations - maybe extend some of the models proposed there
// Continuous formuation done by Beasley for aircraft landing - if A takes of at Ta and B takes off after A, then Ta + (sep between A and b) <= Tb
// Working with Beasley's formuation might be easier
// Furini
// Lieder

// 2. Investigate B&B
// Good starting point
// Integrate within a rolling window

// 3. Comparison with mathematical programming
// Extend Sintef's formulation

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

// Partial integrated = using some of the constraints from problem A to solve problem B
// Barry Smith
// Impose one constraint or characteristic manipulated in an earlier step to improve a later step
// Flight scheduling, fleet assignment, aircraft routing, crew scheduling
// FS = match capacity to demands (lot of demand for a route = put a big aircraft on it)
// FA = important for Crew Sched since crews can only work on certain aircraft
// Station purity = minimise number of aircraft types per hub/airport, fleet sched cost went up but crew connection times went down, good example of partial integration

// Mathematical Modelling
// Model Building in Mathematical Programming, 5e

// TODO: Ask if we will be looking at both takeoffs and landings or just one of the two

// 13-10-2023

// Put first line of objectives into intro
// 4 different solutions to the integrated problem

// Heuristic decisions:
// Sequential
// De-icing based on TOBT
// De-icing based on CTOT
// De-icing based on runway seqeuncing

// Methods:
// Mathematic programming (relatively easy)
// Branch and Bound (tried and tested)
// Rolling Window
// Dynamic programming (we know that this is good, but have never integrated Pushback Sequencing into it, stretch goal)

// How am I going to approach the probleM? (What order are aircraft going to de-ice in?)
// How am I going to solve that problem? (B&B, DP, MP)

// Decomposed (solve either RS or DI first and then solution of the first goes into the second)
// Partial Integration (when solving A independently we take into account side factors that we know will give us an improved solution for B)

// Leave "naive" out

// De-icing should be integrated into the algorithms right from the beginning
// It is a key goal so not right to only start it from the second semester

// Visualisation tool, evaluation, and final diss will probably be throughout the year

// For de-icing we want stable solutions

// Intro = these are the problems, this is how it works, this is why it's important to combine them