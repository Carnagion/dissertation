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

This project explores the integrated version of the aircraft runway sequencing and de-icing problem, which consists of assigning runways, take-off or landing times, and de-icing times to each aircraft from a given set, in a way that complies with strict safety and operational requirements while minimising operational costs, fuel emissions, flight delays, and crew connection times.

This is important because there have been few efforts until now to tackle runway sequencing and de-icing in an integrated fashion -- indeed, existing solutions focus on generating either runway sequences or de-icing schedules only and combining them afterwards.

// What this project is ... "Integrated runway sequencing and de-icing" ...

// What runway sequencing and de-icing is and why it's important ...

// Although it is possible to construct additional runways or airports, it may not always be feasible due to the high infrastructure and planning costs and the (lack of) availability of land. Therefore, efficient scheduling of runway operations is crucial in maximising the capacity of existing runways and airports.

// This requires careful consideration of multiple factors including the type of operation (i.e. taking off or landing), aircraft class of the preceding and succeeding operation, the allocated time frame for the operation, and the number of available runways during that time. When an aircraft takes off or lands, it produces air turbulence that affects the following aircraft. The impact of this turbulence depends on the aircraft's class, which is based on its size and weight. // FROM: Lieder, dynamic programming
// Aircraft are also assigned a specific window of time for taking off or landing, based on surrounding air traffic. // FROM: Lieder, scheduling aircraft
// Furthermore, each crew member may also only be trained to operate certain kinds of aircraft, and may be forced to wait for long periods of time after completing a flight if another aircraft of the same kind is not yet available for them. // TODO: Source?
// Runway sequences must therefore meet strict separation requirements that depend on the aforementioned factors, while minimising operational costs, fuel emissions, flight delays, and crew wait time. // FROM: Lieder, scheduling aircraft

// However, de-icing presents an additional challenge to this.

// TODO: Talk about crew and fleet scheduling

// TODO: Talk about fundamental insights

// TODO: Reconsider use of the word "optimal"?

= Objectives

// TODO: Clean up

The primary aim of this project is to investigate the integrated runway sequencing and de-icing problem by developing three algorithms that explore four different approaches to the order of aircraft de-icing. This will provide a deeper insight into the problem's fundamental characteristics and the interactions between runway sequencing and de-icing, as well as the potential benefits of integrating their solutions.

// TODO: Should insights be mentioned above?

Its key objectives are thus as follows:

1. *Investigate previous approaches to runway sequencing*. The mathematical models and formulations proposed in prior research may not be directly applicable to this project, as there have been few efforts until now to tackle runway sequencing and de-icing in an integrated fashion. Thus, there will be a need to understand and then adapt or extend these models so they are suitable for the integrated problem.

2. *Design and implement three algorithms* -- B&B, B&B with a rolling window, and MP -- *using four different de-icing ordering approaches* -- sequential, based on COBT, based on CTOT, and based on runway sequences. The algorithms must be generic enough to work with data from different sources (i.e. different airports and datasets), by using a set of common features and characteristics in the data. Additionally, they must be fast and reliable enough to be viable in highly dynamic, real-time situations where unexpected failure is not an option. If time permits, a fourth algorithm -- dynamic programming -- may also be explored, since it is known to work well for runway sequencing but its effectiveness at de-icing is yet to be evaluated.

3. *Analyse the algorithms' performance and outputs*. This will involve benchmarking them on known and available datasets, and comparing them with existing solutions as well as with each other. A simulation that is more representative of real-world data and use cases will also be used to run the algorithms on multiple problem instances over a longer period of time. This will help expose any issues, such as instability in the generated sequences, that may not be visible in individual runs.

4. *Develop a tool for visualising the outputs and intermediate results produced by the algorithms*. This will provide a more intuitive, human-friendly view intended to aid users' understanding, which will not only be useful for an end user, but also for the analysis of the algorithms themselves.

= Plan

// TODO: Clean up

The overall work plan is to start off by investigating previous approaches to the problem and establishing a mathematical model, as any further work will be reliant on this. Then, the branch-and-bound algorithm to solve the problem according to the model will be implemented and later extended with a rolling window, followed by the mathematical programming and dynamic programming algorithms.

Analysis and evaluation of the implemented algorithms will take place throughout the development process. The development of the visualisation tool will therefore also start early in order to assist with the analysis.

Likewise, the document deliverables --- the project proposal, interim report, and final dissertation --- will be worked on throughout the project's timeline to enable noting down the tasks carried out and key observations during the year. This will help prevent crunch time closer to their deadlines.

An outline of this plan is depicted in the following Gantt chart:

/ A: Write the project proposal
/ B: Research previous approaches into runway sequencing and de-icing
/ C: Implement a branch-and-bound algorithm
/ D: Develop the visualisation tool
/ E: Evaluate the performance of the algorithm and run simulations
/ F: Write the interim report
/ G: Extend the branch-and-bound algorithm with a rolling window
/ H: Implement a mathematical programming algorithm
/ I: Write the final dissertation
/ J: Christmas break
/ K: Prepare for exams
/ L: Implement a dynamic programming algorithm
/ M: Easter break

#import "@preview/timeliney:0.0.1": *

#timeline(show-grid: true, {
    let months = ("Oct", "Nov", "Dec", "Jan", "Feb", "Mar", "Apr")

    let day(day) = {
        let avg-month-days = 213 / 7
        day / avg-month-days
    }

    let proposal-day = day(26)
    let interim-day = day(69)
    let diss-day = day(203)

    headerline(group(([*2023*], 3)), group(([*2024*], 4)))
    headerline(..months.map(group))

    let break-line-style = (stroke: 3pt + gray)
    let doc-line-style = (stroke: 3pt + gray.darken(25%))
    let work-line-style = (stroke: 3pt)

    // Note: Each month assumed to be approximately 30 days
    taskgroup({
        // Write the project proposal
        task("A", (0, proposal-day), style: doc-line-style)

        // Research previous approaches into runway sequencing and de-icing
        task("B", (0, day(39)), style: work-line-style)

        // Implement a branch-and-bound algorithm
        task("C", (day(20), day(31)), style: work-line-style)

        // Develop the visualisation tool
        task("D", (day(20), day(175)), style: work-line-style)

        // Evaluate the performance of the algorithm and run simulations
        task("E", (day(20), day(182)), style: work-line-style)

        // Write the interim report
        task("F", (proposal-day, interim-day), style: doc-line-style)

        // Extend the branch-and-bound algorithm with a rolling window
        task("G", (day(31), day(52)), style: work-line-style)

        // Implement a mathematical programming algorithm
        task("H", (day(61), day(133)), style: work-line-style)

        // Write the final dissertation
        task("I", (interim-day, diss-day), style: doc-line-style)

        // Christmas break
        task("J", (day(77), day(107)), style: break-line-style)

        // Prepare for exams
        task("K", (day(84), day(119)), style: break-line-style)

        // Implement a dynamic programming algorithm
        task("L", (day(140), day(168)), style: work-line-style)

        // Easter break
        task("M", (day(181), day(212)), style: break-line-style)
    })

    let milestone-line-style = (stroke: (dash: "dashed"))

    milestone(
        at: proposal-day,
        style: milestone-line-style,
        align(center)[
            *Project Proposal*\
            26 Oct
        ],
    )

    milestone(
        at: interim-day,
        style: milestone-line-style,
        align(center)[
            *Interim Report*\
            8 Dec
        ],
    )

    milestone(
        at: diss-day,
        style: milestone-line-style,
        align(center)[
            *Final Dissertation*\
            19 Apr
        ],
    )
})

This plan also includes gaps after the implementations of the rolling window and mathematical programming, and towards the very end just before the Easter break, where no work other than writing documents and analysing the algorithms is carried out. This attempts to account for unexpected delays to the schedule such as underestimation of the time or work involved, slowdown due to Christmas break and exams, or unforseen issues in the implementations.

= References

// TODO: Add citations where needed