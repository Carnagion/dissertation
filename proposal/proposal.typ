#let email(email) = link("mailto:" + email, raw(email))

#set text(font: "EB Garamond", size: 11pt)

#v(1fr)
#align(center)[
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

    #text(size: 13pt)[_COMP3003 Project Proposal_]

    #v(0.2fr)

    #stack(dir: ltr, spacing: 1fr)[
        _By_\
        Indraneel Mahendrakumar\
        20372495\
        #email("psyim3@nottingham.ac.uk")\
    ][
        _Supervised By_\
        Geert De Maere\
        Assistant Professor\
        #email("geert.demaere@nottingham.ac.uk")\
    ]

    #v(0.2fr)

    MSci. Hons. Computer Science with Artificial Intelligence\
    University of Nottingham\
    2023-2024
]
#v(1fr)

// NOTE: Done after cover page since we don't want page numbers to show up on it
#set page(numbering: "1")

#set heading(numbering: "1.1")
#show heading: set block(above: 2em, below: 1.3em)

#set par(justify: true)

#outline(indent: auto)
#pagebreak()

= Introduction

This project explores the integrated version of the aircraft runway sequencing and de-icing problem. It is an NP-hard problem @demaere-pruning-rules which involves assigning runways, take-off or landing times, and de-icing times to each aircraft from a given set in a way that complies with safety and operational requirements @lieder-scheduling-aircraft while minimising operational costs, fuel emissions, flight delays, and crew connection times.

== Background

Aircraft taking off from or landing on a given airport must adhere to strict separation requirements that are dictated by the type of operation (i.e., taking off or landing), the aircraft classes of the preceding and succeeding operations, and the allocated time frame for the operation #cite("lieder-scheduling-aircraft", "lieder-dynamic-programming"). De-icing must also be accounted for -- aircraft may be de-iced inside gates or at de-icing pads, which pushes back the take-off time of the aircraft (and consequently, those of the rest of the sequence) depending on the number of de-icing stations or rigs available at the time.
An airport's maximum capacity and throughput -- the number of aircraft taking off or landing per unit of time -- is thus bounded by its runway capacity @lieder-dynamic-programming. Although it is possible to construct additional runways or airports, this is not always feasible due to the high costs of infrastructure and land. Therefore, efficient use and scheduling of runway operations is crucial for maximising the capacity of existing runways and airports #cite("lieder-scheduling-aircraft", "lieder-dynamic-programming").

== Motivation

Prior approaches to runway sequencing have employed a variety of methods -- both exact and heuristic-based -- such as first-come-first-serve (FCFS) @furini-rolling-horizon, branch-and-bound, linear programming (LP) based tree search @beasley-scheduling-aircraft, dynamic programming #cite("lieder-scheduling-aircraft", "lieder-dynamic-programming"), and mixed-integer programming (MIP) #cite("lieder-dynamic-programming", "avella-time-indexed"). Some have also incorporated a rolling horizon to lower the exponential computation time required for large problem instances #cite("furini-rolling-horizon", "beasley-scheduling-aircraft").

However, these approaches have focused primarily on generating optimal runway sequences or de-icing schedules in isolation or in a decomposed manner (i.e., generating solutions for the two problems independently of each other). There is a possibility that integrating the solutions of runway sequencing and de-icing yields more optimal results, and as such, the problem is ripe for investigation.
This project is thus one of the first of its kind, and investigates four distinct approaches to determining the order of de-icing using three different algorithms.

In doing so, this project will provide fundamental insights into the innate characteristics of and interactions between runway sequencing and de-icing -- which can then be used as a starting point for further research. Additionally, it will reveal the potential advantages of an integrated solution, as compared to using fully decomposed or partially integrated methods proposed in existing literature.

= Objectives

The primary aim of this project is to investigate the integrated runway sequencing and de-icing problem by developing three algorithms that explore four different approaches to the order of aircraft de-icing. The investigation and implementation of these algorithms will provide deeper insights into the problem's fundamental characteristics and the interactions between runway sequencing and de-icing, as well as the potential benefits of integrating their solutions.

The project's key objectives are as follows:

1. *Investigate prior approaches to runway sequencing*. The mathematical models and formulations proposed in prior research may not be directly applicable to this project, as they focus on only runway sequencing or only de-icing. Thus, there will be a need to understand and then adapt or extend these models so they are suitable for the integrated problem.

2. *Design and implement three algorithms* -- branch-and-bound, branch-and-bound with a rolling window, and mathematical programming -- *using four different de-icing ordering approaches* -- sequential, based on the Target Off-Block Time (TOBT), based on the Calculated Take-Off Time (CTOT), and based on runway sequences. The algorithms must be generic enough to work with data from different sources (i.e., different airports and datasets), by using a set of common features and characteristics in the data. Additionally, they must be fast and reliable enough to be viable in highly dynamic, real-time situations where unexpected failure is not an option @demaere-pruning-rules. If time permits, a fourth algorithm -- dynamic programming -- may also be explored, since it is known to work well for runway sequencing @lieder-dynamic-programming but its effectiveness at de-icing is yet to be evaluated.

3. *Analyse the algorithms' performance and outputs*. This will involve benchmarking them on known and available datasets and comparing them with existing solutions as well as with each other. A simulation that is more representative of real-world data and use cases will also be used to run the algorithms on multiple problem instances over a longer period of time. This will help expose any issues, such as instability in the generated sequences, that may not be visible in individual runs.

4. *Develop a tool for visualising the outputs and intermediate results produced by the algorithms*. This will provide a more intuitive, human-friendly view intended to aid users' understanding, which will not only be useful for an end user, but also for the analysis of the algorithms themselves.

= Plan

The overall work plan is to first investigate prior approaches to the problem and establish a mathematical model, as any further work will be reliant on this. Then, the branch-and-bound algorithm to solve the problem according to the model will be implemented and later extended with a rolling window, followed by the mathematical programming and dynamic programming algorithms.

Analysis and evaluation of the implemented algorithms will take place throughout the development process. The development of the visualisation tool will therefore also start early in order to assist with the analysis.

Likewise, the document deliverables --- the project proposal, interim report, and final dissertation --- will be worked on throughout the project's timeline to enable noting down the tasks carried out and key observations during the year. This will help prevent crunch time closer to their deadlines.

An outline of this plan is depicted in the following Gantt chart:

/ A: Write the project proposal
/ B: Research prior approaches into runway sequencing and de-icing
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
    // NOTE: Technically 28 weeks and 5 days
    let num-weeks = 29
    let days-in-week = 7

    let day(day) = day / days-in-week

    let proposal-day = day(25)
    let interim-day = day(72)
    let diss-day = day(201)

    // NOTE: 0.0001 subtracted because timeliney gets stuck if the group length exceeds total length even by a little
    headerline(group(([*2023*], day(92))), group(([*2024*], num-weeks - day(92) - 0.0001)))
    headerline(
        group(("Oct", day(31))),
        group(("Nov", day(30))),
        group(("Dec", day(31))),
        group(("Jan", day(31))),
        group(("Feb", day(29))),
        group(("Mar", day(31))),
        group(("Apr", day(20) - 0.0001)), // NOTE: See above
    )
    headerline(..range(1, num-weeks + 1).map(week => group(str(week))))

    let break-line-style = (stroke: 3pt + gray)
    let doc-line-style = (stroke: 3pt + gray.darken(25%))
    let work-line-style = (stroke: 3pt)

    taskgroup({
        // Write the project proposal
        task("A", (0, proposal-day), style: doc-line-style)

        // Research prior approaches into runway sequencing and de-icing
        task("B", (0, day(49)), style: work-line-style)

        // Implement a branch-and-bound algorithm
        task("C", (day(21), day(31)), style: work-line-style)

        // Develop the visualisation tool
        task("D", (day(21), day(175)), style: work-line-style)

        // Evaluate the performance of the algorithm and run simulations
        task("E", (day(21), day(182)), style: work-line-style)

        // Write the interim report
        task("F", (proposal-day, interim-day), style: doc-line-style)

        // Extend the branch-and-bound algorithm with a rolling window
        task("G", (day(31), day(49)), style: work-line-style)

        // Implement a mathematical programming algorithm
        task("H", (day(56), day(126)), style: work-line-style)

        // Write the final dissertation
        task("I", (interim-day, diss-day), style: doc-line-style)

        // Christmas break
        task("J", (day(77), day(107)), style: break-line-style)

        // Prepare for exams
        task("K", (day(84), day(119)), style: break-line-style)

        // Implement a dynamic programming algorithm
        task("L", (day(133), day(168)), style: work-line-style)

        // Easter break
        task("M", (day(181), day(203)), style: break-line-style)
    })

    let milestone-line-style = (stroke: (dash: "dashed"))

    milestone(
        at: proposal-day,
        style: milestone-line-style,
        align(center)[
            *Project Proposal*\
            25 Oct
        ],
    )

    milestone(
        at: interim-day,
        style: milestone-line-style,
        align(center)[
            *Interim Report*\
            11 Dec
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

This plan also includes gaps after the implementations of the rolling window and mathematical programming, and towards the very end just before the Easter break. This attempts to account for unexpected delays to the schedule such as underestimation of the time or work involved, slowdown due to Christmas break and exams, or unforeseen issues in the implementations.

= References

#bibliography("references.yml", title: none)