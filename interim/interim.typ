#import "@preview/lovelace:0.1.0": *
#import "@preview/timeliney:0.0.1": *

// NOTE: Needs to be called at the top of the document to setup lovelace
#show: setup-lovelace

#let todo(message) = raw("// TODO: " + message, block: true, lang: "rust")

#let email(email) = link("mailto:" + email, raw(email))

#set text(font: "EB Garamond", size: 11pt)

#v(1fr)
#align(center)[
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

    #text(size: 13pt)[_COMP3003 Interim Report_]

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

#set math.equation(numbering: "(1)")

#let pseudocode = pseudocode.with(indentation-guide-stroke: 0.1pt)

// TODO: Remove once the double heading bug is fixed - see https://github.com/andreasKroepelin/lovelace/pull/1
#show figure.where(kind: "lovelace"): fig => {
    let booktabbed = block(
        stroke: (y: 1pt),
        inset: 0pt,
        breakable: true,
        width: 100%,
        {
            set align(left)
            block(
                inset: (y: 5pt),
                width: 100%,
                stroke: (bottom: 1pt),
                {
                    strong({
                        fig.supplement
                        sym.space.nobreak
                        counter(figure.where(kind: "lovelace")).display(fig.numbering)
                        if fig.caption != none {
                            [: ]
                        } else {
                            [.]
                        }
                    })
                    if fig.caption != none {
                        fig.caption.body
                    }
                },
            )
            block(
                inset: (bottom: 5pt),
                breakable: true,
                fig.body
            )
        },
    )
    if fig.placement in (auto, top, bottom) {
        place(fig.placement, float: true, booktabbed)
    } else {
        booktabbed
    }
}

#outline(indent: auto)
#pagebreak()

= Introduction

This project explores the integrated version of the aircraft runway sequencing and de-icing problem for a single runway and single de-icing station. It is a known NP-hard problem @demaere-pruning-rules which involves assigning runways, take-off or landing times, and de-icing times to each aircraft from a given set in a way that complies with safety and operational requirements @lieder-scheduling-aircraft while minimising operational costs, fuel emissions, flight delays, and crew connection times.

== Background <background>

Aircraft taking off from or landing on a given airport must adhere to strict separation requirements that are dictated by the type of operation (i.e., taking off or landing), the aircraft classes of the preceding and succeeding operations, and the allocated time frame for the operation @lieder-scheduling-aircraft @lieder-dynamic-programming. De-icing must also be accounted for -- aircraft may be de-iced inside gates or at de-icing pads, which pushes back the take-off time of the aircraft (and consequently, those of the rest of the sequence) depending on the number of de-icing stations or rigs available at the time.
An airport's maximum capacity and throughput -- the number of aircraft taking off or landing per unit of time -- is thus bounded by its runway capacity @lieder-dynamic-programming. Although it is possible to construct additional runways or airports, this is not always feasible due to the high costs of infrastructure and land. Therefore, efficient use and scheduling of runway operations is crucial for maximising the capacity of existing runways and airports @lieder-scheduling-aircraft @lieder-dynamic-programming.

== Motivation

Prior approaches to runway sequencing have employed a variety of methods -- both exact and heuristic-based -- such as first-come-first-serve (FCFS) @furini-improved-horizon, branch-and-bound, mixed-integer linear programming @beasley-scheduling-aircraft, dynamic programming @lieder-scheduling-aircraft @lieder-dynamic-programming, and mixed-integer programming (MIP) @lieder-dynamic-programming @avella-time-indexed. Some have also incorporated a rolling horizon to lower the exponential computation time required for large problem instances @furini-improved-horizon @beasley-scheduling-aircraft.

However, these approaches have focused primarily on generating optimal runway sequences or de-icing schedules in isolation or in a decomposed manner (i.e., generating solutions for the two problems independently of each other). There is a possibility that integrating the solutions of runway sequencing and de-icing yields more optimal results, and as such, the problem is ripe for investigation.
This project is thus one of the first of its kind, and investigates four distinct approaches to determining the order of de-icing using three different algorithms.

In doing so, this project will provide fundamental insights into the innate characteristics of and interactions between runway sequencing and de-icing -- which can then be used as a starting point for further research. Additionally, it will reveal the potential advantages of an integrated solution, as compared to using fully decomposed or partially integrated methods proposed in existing literature.

= Objectives <objectives>

The primary aim of this project is to investigate the integrated runway sequencing and de-icing problem by developing three algorithms that explore four different approaches to the order of aircraft de-icing. The investigation and implementation of these algorithms will provide deeper insights into the problem's fundamental characteristics and the interactions between runway sequencing and de-icing, as well as the potential benefits of integrating their solutions.

The project's key objectives are as follows:

1. *Investigate prior approaches to runway sequencing*. The mathematical models and formulations proposed in prior research may not be directly applicable to this project, as they focus on only runway sequencing or only de-icing. Thus, there will be a need to understand and then adapt or extend these models so they are suitable for the integrated problem.

2. *Design and implement three algorithms* -- branch-and-bound, branch-and-bound with a rolling window, and mathematical programming -- *using four different de-icing ordering approaches* -- sequentially, based on the Target Off-Block Time (TOBT), based on the Calculated Take-Off Time (CTOT), and based on existing runway sequences. The algorithms must be generic enough to work with data from different sources (i.e., different airports and datasets), by using a set of common features and characteristics in the data. Additionally, they must be fast and reliable enough to be viable in highly dynamic, real-time situations where unexpected failure is not an option @demaere-pruning-rules. If time permits, a fourth algorithm -- dynamic programming -- may also be explored, since it is known to work well for runway sequencing @lieder-dynamic-programming but its effectiveness at de-icing is yet to be evaluated.

3. *Analyse the algorithms' performance and outputs*. This will involve benchmarking them on known and available datasets and comparing them with existing solutions as well as with each other. A simulation that is more representative of real-world data and use cases will also be used to run the algorithms on multiple problem instances over a longer period of time. This will help expose any issues, such as instability in the generated sequences, that may not be visible in individual runs.

4. *Develop a tool for visualising the outputs and intermediate results produced by the algorithms*. This will provide a more intuitive, human-friendly view intended to aid users' understanding, which will not only be useful for an end user, but also for the analysis of the algorithms themselves.

// TODO: Maybe find a better heading
= Existing Literature

Early approaches to runway sequencing used by many airports around the world include simple FCFS algorithms optimising for a single objective @bianco-minimizing-time. Although very simple to implement and computationally inexpensive, FCFS strategies are well-known to produce excessive delays @bianco-minimizing-time. Therefore, a number of more optimising approaches -- using both exact and heuristic-based methods -- have been proposed in the past.

== Heuristic Approaches

// TODO: Talk about heuristic approaches to runway sequencing
#todo("Mention heuristic approaches to runway sequencing")

== Linear Programming

Linearizing the objective function allows the problem to be solved to optimality using mixed-integer linear programming. One such mixed-integer 0-1 formulation is introduced by #cite(<beasley-scheduling-aircraft>, form: "prose") for scheduling aircraft landings, supporting both single or multiple runways operating in either mixed or segregated modes.

Unlike many previous approaches that assumed an indefinite latest time limit for landing, this approach employs more realistic latest landing times based on fuel considerations @beasley-scheduling-aircraft. This allows exploiting the presence of increased disjoint intervals -- caused by relatively narrower hard time windows for arrivals -- to simplify the problem using landing times @demaere-pruning-rules @beasley-scheduling-aircraft.

The approach also allows for complex and arbitrary separation matrices, and is capable of working with different complex objective functions -- both linear and non-linear as long as time can be discretized -- making it applicable to a wider variety of situations.

== Dynamic Programming

Dynamic programming has been used in many solutions in the past @demaere-pruning-rules @psaraftis-dynamic-programming @bianco-minimizing-time @balakrishnan-runway-operations, since it is known to work well for runway sequencing as mentioned in @objectives, and can yield optimal schedules significantly faster than MIP solvers @lieder-dynamic-programming.

#cite(<lieder-dynamic-programming>, form: "prose") provide an optimisation algorithm for runway sequencing based on that of #cite(<briskorn-aircraft-landing>, form: "prose") with more general assumptions -- multiple runways, positive target times, and limited time windows, building upon existing approaches that rely on more restricted or impractical assumptions.

#cite(<bianco-minimizing-time>, form: "prose") present a dynamic programming approach for the single-machine scheduling problem with sequence-dependent setup times. This is equivalent to the runway sequencing problem for a single runway, not taking into account aircraft classes @lieder-dynamic-programming @bianco-minimizing-time. By viewing aircraft as jobs and runways as machines, runway sequencing can be considered a variation of the machine/job scheduling problem, and insights from the latter can be applied to solve the former.

#cite(<psaraftis-dynamic-programming>, form: "prose") utilizes an approach that grouped aircraft into multiple sets, allowing the exploitation of known precedence orders within these sets. When implemented as a preprocessing step, this reduced the problem's worst-case computational complexity to $O(m^2(n + 1)^m)$, where $n$ denotes the number of sets and $m$ denotes the number of aircraft @demaere-pruning-rules @psaraftis-dynamic-programming. This approach is also used in this project and is discussed later in @complete-orders.

#cite(<demaere-pruning-rules>, form: "prose") further introduce a set of pruning principles that exploit the inherent characteristics of the runway sequencing problem including conditional and complete orders (introduced earlier by #cite(<psaraftis-dynamic-programming>, form: "prose")), insertion dominance, dominance with lower bounding, and considering subsets and non-identical sets.

These pruning rules enable significant reductions of the problem's average computational complexity without compromising the optimality of the generated sequences. When integrated into a dynamic program, they have been shown to be able to generate optimal sequences for large instances at a low computational cost. Furthermore, the dynamic program has the ability to consider complex non-linear and non-convex objective functions that model real-world constraints and situations @demaere-pruning-rules.

== Constrained Positional Shifts

A number of solutions -- such as that of #cite(<psaraftis-dynamic-programming>, form: "prose") and #cite(<balakrishnan-runway-operations>, form: "prose") -- have also employed Constrained Positional Shifting (CPS). CPS restricts the shift in position of an aircraft's scheduled departure relative to its original position in the initial sequence, typically an (unoptimised) FCFS sequence. Not only does this prune the search space by reducing the number of aircraft that must be considered for each position in the sequence, but it also encourages fairness by preventing aircraft from being advanced or delayed disproportionately relative to other aircraft @demaere-pruning-rules.

However, CPS may be impractical in situations involving CTOTs or other time window constraints, or mixed-mode operations (i.e. both arrivals and departures on the same runway) where delays between arrivals and departures may differ widely. These can require large positional shifts, thereby challenging the tractability of CPS-based approaches @demaere-pruning-rules.

= Design

// TODO: Find something to put here
#todo("Have a paragraph here to introduce the rest of the section")

== Data

An initial dataset of instances was needed to test the implementation on. These were obtained from the University of Bologna Operations Research Group's freely accessible online #link("https://site.unibo.it/operations-research/en/research/library-of-codes-and-instances-1")[library of instances]. These instance sets consisted of rows of aircraft with their registration numbers, models, weight class, operation type (arrival or departure), and earliest possible take-off time, as well as the separations between each pair of aircraft. The instances were also used for testing in previous works @furini-improved-horizon. // TODO: Reference Furini's earlier paper

=== Data Generation

The datasets chosen were meant to be used in the runway sequencing problem, not integrated runway and de-icing sequencing. This meant that the instances did not contain data for the pushback durations, taxi durations, de-icing durations, and line-up durations of aircraft, making them largely unsuitable for use as-is in this project. Thus, there was a need to augment the data and create a dataset generator.

// TODO: Talk about creating the new CSV format and generating random data

== Aircraft Separations

As mentioned in @background, each aircraft must adhere to strict separation requirements that enforce a minimum waiting time before taking off after the previous aircraft. These separations are defined by the appropriate aviation authorities by classifying aircraft into a number of classes -- typicaly based on size or weight -- and specifying the separation that must apply between each class @beasley-scheduling-aircraft. Many of the existing works on runway sequencing assume that there are a fixed number of aircraft classes, and that these are the only factors influencing separation times @beasley-scheduling-aircraft.

In practice, however, separation times are decided based on a number of other factors. For example, at London Heathrow, separation times relate not only to aircraft classes but also to the Standard Instrument Departure (SID) route that the aircraft is to follow after take-off @beasley-scheduling-aircraft. Assuming a fixed number and mapping of aircraft classes to separation durations would therefore fail to generalise to every single system or practical situation.

To cater to such situations, this project makes no such assumptions, and the data structures and representations used allow for unique separations between each pair of aircraft that are to be sequenced.

== Objective Function

For this problem, the objective function $f$ represents the total cost of a sequence of departures $D$ in terms of its delays. This can be expressed as the sum of each scheduled departure's deviation from the earliest possible take-off time for that aircraft:

$
f(D) = sum_(x in D) (T_x - E_x)^2 
$ <objective-function-equation>

The longer the deviation and number of deviations in $D$, the higher the objective value will be. Thus, the problem is one of minimisation, i.e. finding the runway sequence with the minimum objective value, which translates to the minimum possible delay.

Note that the difference (in minutes) between an aircraft $x$'s scheduled take-off time $T_x$ and its earliest possible take-off time $E_x$ is squared. This ensures fairness by favouring moderate delays for all aircraft rather than exceedingly high delays for some and little to no delays for the rest.

// TODO: Illustrate the above with an example
#todo("Add an example to illustrate this")

= Implementation

// TODO: Review and check if the Rust website should be cited
For this project, I have opted to use #link("https://www.rust-lang.org")[Rust]. The primary reason for this is my familiarity and experience with the language, which allows me to be more confident in my implementation and estimated timelines. Another major factor is that Rust's rich type system and unique memory ownership and borrowing mechanics eliminate many classes of bugs -- such as null reference exceptions or Undefined Behaviour -- at compile time. As a result, I can be more productive while being confident in my implementation's reliability and handling of edge cases.

== Complete Orders <complete-orders>

Before sequencing, an instance is split into sets of _separation-identical_ aircraft as a preprocessing step. Two aircraft $x$ and $y$ are separation-identical if their mutual separations with respect to every other aircraft $z$ in the set of aircraft $A$ are the same @demaere-pruning-rules @psaraftis-dynamic-programming; i.e. $x$ and $y$ are separation-identical if and only if:

$
forall_(z in A), z != x and z != y and delta_(x z) = delta_(y z) and delta_(z x) = delta_(z y)
$

Separation-identical sets are generated by comparing the separations of every pair of aircraft with every other aircraft in $A$ as follows:

// TODO: Maybe add some comments
#algorithm(
    caption: [Calculating sets of separation-identical aircraft],
    pseudocode(
        no-number,
        [*input*: set of aircraft $A$],
        no-number,
        [*output*: separation-identical sets of aircraft in $A$],

        [$S <-$ empty list],
        [*for* $x$ *in* $A$ *do*], ind,
            [*for* $s$ *in* $S$ *do*], ind,
                [*for* $y$ *in* $s$ *do*], ind,
                    [*for* $z$ *in* $A$ except $x, y$ *do*], ind,

                        [*if* $delta_(x z) != delta_(y z)$ *or* $delta_(z x) != delta_(z y)$ *then*], ind,
                            [*continue* to next set in $S$], ded,
                        [*end*], ded,

                    [*end*], ded,
                [*end*],
                
                [add $x$ to $s$],
                [*continue* to next aircraft in $A$], ded,

            [*end*],

            [$s <-$ singleton with $x$],
            [add $s$ to $S$], ded,

        [*end*], ded,

        [*return* $S$],
    ),
)

This allows the exploitation of _complete orders_ between separation-identical aircraft. A complete order exists between two aircraft $x$ and $y$ if any arbitrary sequence containing $x$ and $y$ cannot be improved any further by reversing the orders of $x$ and $y$ @demaere-pruning-rules. Such complete orders simplify the problem of runway sequencing to one of interleaving ordered sets of separation-identical aircraft. It also reduces the problem's worst-case computational complexity from $n!$ to $O(m^2(n + 1)^m)$, where $n$ denotes the number of sets and $m$ denotes the number of aircraft @demaere-pruning-rules @psaraftis-dynamic-programming.

Since all of the methods used in this project are exact methods, using separation-identical sets does not compromise the optimality of the generated sequences @demaere-pruning-rules @psaraftis-dynamic-programming, and considerably trims the solution search space.

At the same time, the efficiency of exploiting complete orders is highly dependent on the separations between aircraft and the diversity of aircraft. In practice, complete orders can be exploited well due to the typical separation matrices and aircraft diversity in runway sequencing instances -- this was the case for the test instances as well. However, in some cases -- such as when every aircraft is subject to a CTOT or when there are very few separation-identical aircraft -- the number of sets might be too large and the number of aircraft in each set too small. Such cases prevent the effective exploitation of complete orders @demaere-pruning-rules.

== Branch-and-bound

// TODO: Tweak as this is taken from Wikipedia
Branch-and-bound is a method for solving optimisation problems by breaking them down into smaller sub-problems and using a _bounding_ function to eliminate those sub-problems that cannot possibly contain a more optimal solution than the best known optimal one found so far.

// TODO: Review this entire section

First, the algorithm starts off with an empty sequence and a best known objective value of infinity. It then visits the sub-sequences of the current sequence, each of which contains a unique subset of all possible feasible solutions. This is done by iterating through each set of separation-identical aircraft, picking the next unsequenced aircraft from that set, pushing it to the end of the current sequence, and recursing.

Before recursing on this new branch, however, the upper and lower bounds of the new sub-sequence are calculated. If the sum of these bounds is greater than the best cost obtained so far, the branch is ignored and the newly added aircraft is removed, continuing to the next aircraft or set. This simple check allows the algorithm to prune the solution search space and perform better than a brute-force (exhaustive) search. After the recursive call returns, the aircraft that was pushed to the sequence is removed, and the algorithm continues to the next aircraft or the next set of separation-identical aircraft.

And finally, if the current runway sequence includes all aircraft from $A$, then its objective value is compared to the best known objective value at that point. If it is better, the sequence is set as the best known runway sequence and its objective value to the best known objective value.

To exploit the characteristics of complete orders between sets of separation-identical aircraft as previously mentioned in @complete-orders, the indices of the last-added aircraft in each separation-identical set are passed around as a paramter. By doing so, the algorithm does not ever swap around the orders of two aircraft from the same set, and is able to prune the search space further compared to simply picking the next aircraft from the set of all aircraft.

A full implementation of the branch-and-bound algorithm is as follows:

#algorithm(
    caption: [Branch-and-bound for runway and de-icing sequencing],
    pseudocode(
        no-number,
        [*inputs*: set of aircraft $A$, sets of separation-identical aircraft $S$, indexes $I$ of last included aircraft from each set in $S$, current runway sequence $D$, current objective value $C$, best known objective value $C_b$, best known runway sequence $D_b$],
        no-number,
        [*output*: best runway sequence],

        [*if* length of $D$ $=$ length of $A$ *then*], ind,
            [*if* $C > C_b$ *then*], ind,
                [$C_b <- C$],
                [$D_b <- D$], ded,
            [*end*], ded,
        [*else*], ind,
            [*for* $s, i$ *in* $S$ zipped with $I$ *do*], ind,
                [*if* $i >=$ length of $s$ *then*], ind,
                    [*continue*], ded,
                [*end*],

                [$x <-$ aircraft at index $i$ in $s$],
                [$d <-$ schedule departure for $x$],

                [$c <-$ cost of $d$],
                [*if* $C + c > C_b$ *then*], ind,
                    [*continue*], ded,
                [*end*],

                [add $d$ to $D$],
                [$C <- C + c$],
                [$i <- i + 1$],

                [$D_b <- $ *recurse* with updated parameters],

                [remove $d$ from $D$],
                [$C <- C - c$],
                [$i <- i - 1$], ded,
            [*end*], ded,
        [*end*],
        [*return* $D_b$],
    ),
) <branch-and-bound-pseudocode>

=== Bounding

A sequence's lower bound -- i.e. the best possible value for that sequence, assuming all future aircraft are scheduled with no delays -- can simply be calculated using the objective function as described in @objective-function-equation. The pseudocode for this is as follows:

#algorithm(
    caption: [Objective function for runway sequences],
    pseudocode(
        no-number,
        [*input*: sequence of aircraft departures $D$],
        no-number,
        [*output*: cost of $D$],
        
        [$c <- 0$],
        [*for* $x$ *in* $D$ *do*], ind,
            [$d <- (T_x - E_x)$ in minutes],
            [$c <- c + d^2$], ded,
        [*end*],
        [*return* $c$],
    ),
)

However, it is more efficient to update the bounds of the current sequence in each iteration by passing them around as a parameter as seen in @branch-and-bound-pseudocode. This avoids having to re-calculate them from scratch every iteration and leads to a noticeable decrease in run time, especially for larger instances with more aircraft to sequence.

// TODO: Insert benchmarks to provide evidence
#todo("Insert benchmarks to show the improvement")

To further prune the solution search space, an estimate for the upper bound of a runway sequence is obtained by assigning take-off times to each remaining (yet to be sequenced) aircraft, as outlined in @upper-bound-pseudocode. This assumes a fixed separation of one minute between all of them. De-icing times for these aircraft are also calculated in a similar manner, disregarding the actual duration required to go through the process.

#algorithm(
    caption: [Estimation of the upper bound for a runway sequence],
    pseudocode(
        no-number,
        [*input*: sets of separation-identical aircraft $S$, indexes $I$ of last included aircraft from each set in $S$, most recently scheduled aircraft $x$],
        no-number,
        [*output*: estimated cost for remaining aircraft],

        // TODO: Write pseudocode for upper bound estimation
        todo("Write the pseudocode for upper bound estimation")
    ),
) <upper-bound-pseudocode>

Although this does not always yield an accurate cost, using a small separation and naive scheduling strategy avoids overshooting the actual upper bound, and thus prevents the branch-and-bound algorithm from incorrectly pruning a potentially better sub-sequence.

== De-Icing Order

// TODO: Talk about de-icing order

In the current implementation, each aircraft is assigned a de-icing time based on its TOBT. The TOBT of an aircraft can be calculated as its earliest possible take-off time minus the time taken to get from the gates to the runway, including de-icing and lineup.

== Visualising Sequences

// TODO: Talk about the visualiser implementation
#todo("Write about the visualiser implementation and insert images of the output")

= Progress

As previously mentioned in the Project Proposal, the goals for the first half of the project were to investigate prior approaches to runway sequencing, fully implementing a branch-and-bound algorithm, and developing a visualisation tool for the generated sequences. These goals have been mostly successfully realised -- in its current state, the project has a working branch-and-bound implementation, a basic runway sequence visualisation tool, and a dataset generator.

However, a rolling horizon extension to the algorithm has not yet been implemented. The time taken to produce a basic working branch-and-bound implementation without de-icing was longer than expected -- the original plan allocated approximately two weeks for this, but in reality it required closer to three weeks. This was primarily due to issues with adapting a classic branch-and-bound method to utilizing the preprocessed sets of separation-identical aircraft, and underestimation of the workload of other modules.

Despite this, the project is on schedule since the original plan had accounted for such delays. As mentioned in the proposal, the Gantt chart -- seen below in @original-gantt -- includes gaps of multiple days at various points througout the timeline:

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

#let original-gantt = timeline(show-grid: true, {
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

#figure(original-gantt, caption: [Original Gantt chart], gap: 1em) <original-gantt>

Additionally, certain tasks -- such as developing an initial visualisation tool -- took much less time than expected, further offsetting the delay incurred by the branch-and-bound-implementation. Based on this, the timelines for some remaining tasks have also been revised.

This Gantt chart has now been updated to reflect the actual tasks completed during the first half of the project, as well as the remaining tasks along with their revised timelines set to be completed during the second half:

/ A: Write the project proposal
/ B: Research prior approaches into runway sequencing and de-icing
/ C: Implement a branch-and-bound algorithm
/ D: Write the interim report
/ E: Evaluate the performance of the algorithm and run simulations
/ F: Develop the visualisation tool
/ G: Write the final dissertation
/ H: Extend the branch-and-bound algorithm with a rolling window
/ I: Christmas break
/ J: Prepare for exams
/ K: Implement a mathematical programming algorithm
/ L: Implement a dynamic programming algorithm
/ M: Easter break

#let revised-gantt = timeline(show-grid: true, {
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
        task("B", (0, day(56)), style: work-line-style)

        // Implement a branch-and-bound algorithm
        task("C", (day(25), day(49)), style: work-line-style)

        // Write the interim report
        task("D", (proposal-day, interim-day), style: doc-line-style)

        // Evaluate the performance of the algorithm and run simulations
        task("E", (day(35), day(182)), style: work-line-style)

        // Develop the visualisation tool
        task("F", (day(49), day(175)), style: work-line-style)

        // Write the final dissertation
        task("G", (interim-day, diss-day), style: doc-line-style)

        // Extend the branch-and-bound algorithm with a rolling window
        task("H", (interim-day, day(91)), style: work-line-style)

        // Christmas break
        task("I", (day(77), day(107)), style: break-line-style)

        // Prepare for exams
        task("J", (day(84), day(119)), style: break-line-style)

        // Implement a mathematical programming algorithm
        task("K", (day(98), day(126)), style: work-line-style)

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

#figure(revised-gantt, caption: [Revised Gantt chart], gap: 1em)

// TODO: Find a better place to put this
Weekly meetings were held to discuss the project's progress, plan the tasks for the current week, and clarify any issues encountered. These were very effective at ensuring a constant flow of development without long breaks, and enabled the discovery and discussion of problems relatively early.

== Contributions and Reflections

// TODO: Talk about reflections including LSEPI

= References

#bibliography("references.yml", title: none)