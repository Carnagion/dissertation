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

// NOTE: Workaround to get non-math text to use EB Garamond in math equations until Typst ships a native function for doing so
#let markup(name) = math.text(font: "EB Garamond", weight: "regular", name)

#let pseudocode = pseudocode.with(indentation-guide-stroke: 0.1pt)

#show figure.where(kind: table): set block(breakable: true)

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

Aircraft taking off from or landing on a given airport must adhere to strict separation requirements that are dictated by the type of operation (i.e., taking off or landing), the aircraft classes of the preceding and succeeding operations, and the allocated time frame for the operation @lieder-scheduling-aircraft @lieder-dynamic-programming. De-icing must also be accounted for -- aircraft may be de-iced at gates or at de-icing pads, which pushes back the take-off time of the aircraft (and consequently, those of the rest of the sequence) depending on the number of de-icing stations available at the time.
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

// TODO: Change citation style - eg. "Beasley et al."
= Existing Literature <existing-literature>

Early approaches to runway sequencing used by many airports around the world include simple FCFS algorithms optimising for a single objective @bianco-minimizing-time. Although very simple to implement and computationally inexpensive, FCFS strategies are well-known to produce excessive delays @bianco-minimizing-time. Therefore, a number of more optimising approaches -- using both exact and heuristic-based methods -- have been proposed in the past.

== Heuristic Approaches

#cite(<atkin-hybrid-metaheuristics>, form: "prose") introduce a hybridised metaheuristic approach to aid runway scheduling at London Heathrow. This involves using a Tabu Search metaheuristic to search for good feasible departure orders.

#cite(<bianco-minimizing-time>, form: "prose") introduce two heuristic algorithms -- Cheapest Addition Heuristic (CAH) and Cheapest Insertion Heuristic (CIH). They note that the latter almost always performs better than the former as it searches for the best partial sequences obtained by inserting new times anywhere within the sequence. However, it is also much more computationally expensive @bianco-minimizing-time.

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

// TODO: Use "operation time" or similar and not just "take-off"
= Design

The findings and insights obtained from existing literature detailed in @existing-literature were used to design and implement an initial objective function, branch-and-bound algorithm, and scheduling strategy, with the de-icing order determined by the TOBT of aircraft.

== Notation

The following notation will be used in equations and pseudocode throughout this report:

#{
    // NOTE: This isn't actually an equation and so shouldn't be numbered
    set math.equation(numbering: none)

    $
    &A &= &markup("Set of aircraft to schedule departures and de-icing for")\
    &p_x &= &markup("Pushback duration of aircraft") x\
    &u_x &= &markup("Pre-de-icing taxi duration of aircraft") x\
    &v_x &= &markup("De-icing duration of aircraft") x\
    &w_x &= &markup("Post-de-icing taxi duration of aircraft") x\
    &q_x &= &markup("Lineup duration of aircraft") x\
    &e_x &= &markup("Earliest allocated departure time for aircraft") x\
    &t_x &= &markup("Scheduled (actual) departure time for aircraft") x\
    &delta_(x y) &= &markup("Separation between aircraft") x markup("and") y markup("where") y markup("goes after") x\
    &D &= &markup("Sequence of aircraft with scheduled departures and de-icing times")
    $
}

== Data

An initial dataset of instances was needed to test the implementation on. These were obtained from the University of Bologna Operations Research Group's freely accessible online #link("https://site.unibo.it/operations-research/en/research/library-of-codes-and-instances-1")[library of instances]. These instance sets consist of rows of aircraft with their registration numbers, models, weight class, operation type (arrival or departure), and earliest possible take-off time, as well as the separations between each pair of aircraft. The instances were also used for testing in previous works @furini-improved-horizon.

=== Data Generation <data-generation>

The datasets chosen were meant to be used in the runway sequencing problem, not integrated runway and de-icing sequencing. This meant that the instances did not contain data for the pushback durations, taxi durations, de-icing durations, and line-up durations of aircraft, making them largely unsuitable for use as-is in this project. Thus, there was a need to augment the data and create a dataset generator.

First, a new Comma-Separated Value (CSV) data format was created for these datasets, which included all the relevant fields. This format included both the separation matrices and the rows of aircraft data together in a single CSV file, unlike the old instance sets that separated them into different files. This made the data much easier to parse and save.

A randomisation algorithm was then created to randomise instances after parsing them, allowing for the generation of new data from an existing dataset. This alters the separation times and the pushback, taxi, de-icing, and lineup durations of each aircraft in the instance. Separations are randomised within a specified range as defined by the aircraft's size class -- small aircraft are assigned a separation between one and three minutes, medium-sized aircraft between two and four minutes, and large aircraft either four or five minutes. The pushback, taxi, de-icing, and lineup durations are each assigned a random duration between one to four minutes. All other data -- such as the number of aircraft, their earliest allocated take-off times, their size classes, and so on -- is untouched.

== Aircraft Separations

As mentioned in @background, each aircraft must adhere to strict separation requirements that enforce a minimum waiting time before taking off after the previous aircraft. These separations are defined by the appropriate aviation authorities by classifying aircraft into a number of classes -- typicaly based on size or weight -- and specifying the separation that must apply between each class @beasley-scheduling-aircraft. Many of the existing works on runway sequencing assume that there are a fixed number of aircraft classes, and that these are the only factors influencing separation times @beasley-scheduling-aircraft.

In practice, however, separation times are decided based on a number of other factors. For example, at London Heathrow, separation times relate not only to aircraft classes but also to the Standard Instrument Departure (SID) route that the aircraft is to follow after take-off @beasley-scheduling-aircraft. Assuming a fixed number and mapping of aircraft classes to separation durations would therefore fail to generalise to every single system or practical situation.

To cater to such situations, this project makes no such assumptions, and the data structures and representations used allow for unique separations between each pair of aircraft that are to be sequenced.

== Objective Function

For this problem, the objective function $f$ represents the total cost of a sequence of departures $D$ in terms of its delays. This can be expressed as the sum of the deviation of each scheduled departure $t_x$ from the earliest allocated departure time $e_x$ for that aircraft:

$
f(D) = sum_(x in D) (t_x - e_x)^2 
$ <objective-function-equation>

The longer the deviation and number of deviations in $D$, the higher the objective value will be. Thus, the problem is one of minimisation, i.e. finding the runway sequence with the minimum objective value, which translates to the minimum possible delay.

Note that the difference (in minutes) between an aircraft $x$'s scheduled take-off time $t_x$ and its earliest allocated take-off time $e_x$ is squared. This ensures fairness by favouring moderate delays for all aircraft rather than exceedingly high delays for some and little to no delays for the rest.

= Implementation

For this project, I have opted to use #link("https://www.rust-lang.org")[Rust]. The primary reason for this is my familiarity and experience with the language, which allows me to be more confident in my implementation and estimated timelines. Another major factor is that Rust's rich type system and unique memory ownership and borrowing mechanics eliminate many classes of bugs -- such as null reference exceptions or Undefined Behaviour -- at compile time. As a result, I can be more productive while being confident in my implementation's reliability and handling of edge cases.

== Complete Orders <complete-orders>

Before sequencing, an instance is split into sets of _separation-identical_ aircraft as a preprocessing step. Two aircraft $x$ and $y$ are separation-identical if their mutual separations with respect to every other aircraft $z$ in the set of aircraft $A$ are the same @demaere-pruning-rules @psaraftis-dynamic-programming; i.e. $x$ and $y$ are separation-identical if and only if:

$
forall_(z in A), z != x and z != y and delta_(x z) = delta_(y z) and delta_(z x) = delta_(z y)
$

Separation-identical sets are identified by comparing the separations of every pair of aircraft with every other aircraft in $A$ as follows:

#algorithm(
    caption: [Identifying sets of separation-identical aircraft],
    pseudocode(
        no-number,
        [*input*: set of aircraft $A$],
        no-number,
        [*output*: separation-identical sets of aircraft in $A$],

        [$S <- markup("empty list")$],
        [*for* $x$ *in* $A$ *do*], ind,
            [*for* $s$ *in* $S$ *do*], ind,
                [*for* $y$ *in* $s$ *do*], ind,
                    [*for* $z$ *in* $A markup("except") x, y$ *do*], ind,
                        [*if* $delta_(x z) != delta_(y z) or delta_(z x) != delta_(z y)$ *then*], ind,
                            [*continue* to next set in $S$], ded,
                        [*end*], ded,
                    [*end*], ded,
                [*end*],
                
                [add $x$ to $s$],
                [*continue* to next aircraft in $A$], ded,

            [*end*],

            [$s <- markup("singleton with ") x$],
            [add $s$ to $S$], ded,

        [*end*], ded,

        [*return* $S$],
    ),
)

This allows the exploitation of _complete orders_ between separation-identical aircraft. A complete order exists between two aircraft $x$ and $y$ if any arbitrary sequence containing $x$ and $y$ cannot be improved any further by reversing the orders of $x$ and $y$ @demaere-pruning-rules. Such complete orders simplify the problem of runway sequencing to one of interleaving ordered sets of separation-identical aircraft. It also reduces the problem's worst-case computational complexity from $n!$ to $O(m^2(n + 1)^m)$, where $n$ denotes the number of sets and $m$ denotes the number of aircraft @demaere-pruning-rules @psaraftis-dynamic-programming.

Since all of the methods used in this project are exact methods, using separation-identical sets does not compromise the optimality of the generated sequences @demaere-pruning-rules @psaraftis-dynamic-programming, and considerably trims the solution search space.

At the same time, the efficiency of exploiting complete orders is highly dependent on the separations between aircraft and the diversity of aircraft. In some cases -- such as when every aircraft is subject to a CTOT or when there are very few separation-identical aircraft -- the number of sets might be too large and the number of aircraft in each set too small. Such cases prevent the effective exploitation of complete orders. However, in practice, complete orders can be exploited well due to the typical separation matrices and aircraft diversity in runway sequencing instances @demaere-pruning-rules -- this was the case for the test instances as well.

== Scheduling Individual Aircraft

Given an aircraft $x$, the earliest possible time it can take off is the maximum of its allocated earliest time $e_x$ and the previous aircraft $w$'s actual take-off time $t_w$ plus the mandatory separation $delta_(w x)$ required between them. If there is no previous aircraft, then $x$ is the first aircraft to be scheduled and its earliest possible take-off time is simply the earliest allocated take-off time $e_x$. Once calculated, this can be used to update the aircraft's TOBT.

Its earliest possible de-icing time can then be calculated as the maximum of the time the previous aircraft $w$ finishes de-icing and the time that $x$ can actually arrive at the de-icing station, considering its updated TOBT. If there is no previous aircraft, then its earliest possible de-icing time is simply the time it needs to start de-icing to meet its earliest allocated take-off time $e_x$.

Finally, its _actual_ take-off time $t_x$ can be calculated as the maximum of its earliest possible take-off time and the time that $x$ can arrive at the runway. The latter can be expressed as its de-icing time plus its de-icing duration, taxi duration, and runway lineup duration.

The pseudocode for this scheduling process is shown below:

#algorithm(
    caption: [Scheduling an aircraft's de-icing and departure times],
    pseudocode(
        no-number,
        [*input*: aircraft $x$, sequence of aircraft departures $D$],
        no-number,
        [*output*: de-icing and departure times for $x$],
        
        [$w <- markup("last scheduled aircraft in") D$],
        [*if* $w markup("exists")$ *then*], ind,
            [$e <- markup("maximum of") e_x markup("and") (t_w + delta_(w x))$],
            [$d <- markup("maximum of") (d_w + v_w) markup("and") (e - (v_x + w_x + q_x))$],
            [$t <- markup("maximum of") e markup("and") (d + v_x + w_x + q_x)$],
            [*return* $d, t$], ded,
        [*else*], ind,
        	[*return* $e_x - (v_x + w_x + q_x), e_x$], ded,
        [*end*],
    ),
)

== Branch-and-Bound Algorithm

Branch-and-bound is a method for solving optimisation problems by breaking them down into smaller sub-problems and using a _bounding_ function to eliminate those sub-problems that cannot possibly contain a more optimal solution than the best known optimal one found so far. The use of the bounding function allows the algorithm to prune the solution space and perform better than a brute-force (exhaustive) search.

This version of branch-and-bound exploits the characteristics of complete orders between sets of separation-identical aircraft as previously mentioned in @complete-orders by passing around the indices of the last-added aircraft in each separation-identical set as a paramter. By doing so, the algorithm does not ever swap around the orders of two aircraft from the same set, and is able to prune the search space further than if it were simply picking the next aircraft from the set of all aircraft.

A full implementation of the branch-and-bound algorithm is as follows:

#algorithm(
    caption: [Branch-and-bound for runway and de-icing sequencing],
    pseudocode(
        no-number,
        [*inputs*: set of aircraft $A$, sets of separation-identical aircraft $S$, indexes $I$ of last included aircraft from each set in $S$, current runway sequence $D$, current objective value $C$, best known objective value $C_b$, best known runway sequence $D_b$],
        no-number,
        [*output*: best runway sequence],

        [*if* $markup("length of") D = markup("length of") A$ *then*], ind,
            [*if* $C > C_b$ *then*], ind,
                [$C_b <- C$],
                [$D_b <- D$], ded,
            [*end*], ded,
        [*else*], ind,
            [*for* $s, i$ *in* $S markup("zipped with") I$ *do*], ind,
                [*if* $i >= markup("length of") s$ *then*], ind,
                    [*continue*], ded,
                [*end*],

                [$x <- markup("aircraft at index") i markup("in") s$],
                [$d <- markup("schedule departure for") x$],

                [$c <- markup("cost of") d$],
                [*if* $C + c > C_b$ *then*], ind,
                    [*continue*], ded,
                [*end*],

                [add $d$ to $D$],
                [$C <- C + c$],
                [$i <- i + 1$],

                [$D_b <- markup("recurse with updated parameters")$],

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
            [$d <- (t_x - e_x) markup("in minutes")$],
            [$c <- c + d^2$], ded,
        [*end*],
        [*return* $c$],
    ),
)

However, it is more efficient to update the bounds of the current sequence in each iteration by passing them around as a parameter as seen in @branch-and-bound-pseudocode. This avoids having to re-calculate them from scratch every iteration and leads to a noticeable decrease in run time, especially for larger instances with more aircraft to sequence.

To further prune the solution search space, an estimate for the upper bound of a runway sequence is obtained by assigning take-off times to each remaining (yet to be sequenced) aircraft, as outlined in @upper-bound-pseudocode. This assumes a fixed separation of one minute between all of them. De-icing times for these aircraft are also calculated in a similar manner, disregarding the actual duration required to go through the process.

#algorithm(
    caption: [Estimation of the upper bound for a runway sequence],
    pseudocode(
        no-number,
        [*input*: set of aircraft $A$, sequence of departures $D$, most recently scheduled aircraft $x$],
        no-number,
        [*output*: estimated cost for remaining aircraft],

        [$c <- 0$],
        [$t <- t_x$],
        [*for* $y$ *in* $A$ *do*], ind,
            [*if* $y in.not D and y != x$ *then*], ind,
                [$t <- markup("maximum of") t_y markup("and") (t + 1 markup("minute"))$],
                [$c <- c + (t - e_y)^2$], ded,
            [*end*], ded,
        [*end*],
        [*return* $c$],
    ),
) <upper-bound-pseudocode>

Although this does not always yield an accurate cost, using a small separation and naive scheduling strategy avoids overshooting the actual upper bound, and thus prevents the branch-and-bound algorithm from incorrectly pruning a potentially better sub-sequence.

== Results

Shown below in @benches-furini are the computational costs (in seconds) for the aforementioned branch-and-bound algorithm benchmarked on twelve instances with 10 aircraft each. These were obtained by taking 10 aircraft from, augmenting, and then randomising the instances introduced in #cite(<furini-improved-horizon>, form: "prose"), using the randomisation process described in @data-generation.

#let benches-furini = table(
    columns: 6,
    [*Instance*], [*Start time*], [*End time*], [*Mean runtime*], [*Fastest runtime*], [*Slowest runtime*],
    [FPT01], [14:55], [15:50], [15.5 ms], [14.83 ms], [28.18 ms],
    [FPT02], [15:30], [16:15], [58.75 ms], [56.14 ms], [91.7 ms],
    [FPT03], [15:47], [16:33], [15.3 ms], [14.44 ms], [28.27 ms],
    [FPT04], [16:14], [17:01], [16.21 ms], [15.02 ms], [30.43 ms],
    [FPT05], [16:35], [17:31], [58.28 ms], [53.05 ms], [85.67 ms],
    [FPT06], [14:02], [14:49], [7.179 ms], [6.794 ms], [8.412 ms],
    [FPT07], [14:32], [15:14], [14.56 ms], [13.4 ms], [25.54 ms],
    [FPT08], [14:55], [15:44], [24.59 ms], [21.31 ms], [45.54 ms],
    [FPT09], [15:25], [16:16], [10.8 ms], [10.07 ms], [19.01 ms],
    [FPT10], [15:55], [16:42], [42.94 ms], [39.45 ms], [62.48 ms],
    [FPT11], [16:28], [17:10], [63.08 ms], [59.43 ms], [78.01 ms],
    [FPT12], [16:45], [17:23], [12.74 ms], [12.06 ms], [20.73 ms],
)

#figure(benches-furini, caption: [Results for subsets of the benchmark instances introduced by #cite(<furini-improved-horizon>, form: "prose")]) <benches-furini>

These measurements were taken on a computer running Windows 10 (64-bit) with an Intel Core i7-10750H 2.60GHz CPU and 32 GB of memory. Each instance's benchmark was sampled 100 times with 100 iterations per sample.

== Visualising Sequences

Alongside the branch-and-bound algorithm, a tool for visualising generated runway sequences has also been developed. The visualiser takes any sequence of departures and de-icing times and produces a Scalable Vector Graphic (SVG) file showcasing the earliest allocated departure time, pushback duration, pre-de-ice taxi duration, scheduled de-icing time, de-ice duration, post-de-ice taxi duration, runway lineup time, and scheduled departure time for each aircraft. The SVG format was chosen because it is a vector graphics format supported by a wide range of browsers and image applications, and because its XML-like syntax makes SVG files easy to create and manipulate within code. An output from the visualiser is shown below:

#figure(image("visual.svg"), caption: [Visualiser output])

Time increases along the horizontal axis, while the aircraft that are sequenced are laid out vertically, from the first to take-off at the top, to the last at the bottom. The different durations are coloured differently to help distinguish them. The black lines represent the scheduled de-icing and departure times, and the dashed lines represent the earliest allocated departure times for each aircraft. Although simple, this output already aids greatly in obtaining a better view and understanding of the generated sequences, and was also invaluable in identifying and eliminating bugs in the branch-and-bound implementation.

As the project progresses, there will likely be a need for different kinds of visualisations and plots -- for example, plotting a tree of intermediate solutions considered by the runway sequencing algorithm. As such, there is a need to continually work on the visualiser and enhance its capabilities.

= Progress

// NOTE: Technically 28 weeks and 5 days
#let num-weeks = 29
#let days-in-week = 7

#let day(day) = day / days-in-week

#let proposal-day = day(25)
#let interim-day = day(72)
#let diss-day = day(201)

As previously mentioned in the Project Proposal, the goals for the first half of the project -- seen below in @original-gantt -- were to investigate prior approaches to runway sequencing, fully implementing a branch-and-bound algorithm, and developing a visualisation tool for the generated sequences. These goals have been mostly successfully realised -- in its current state, the project has a working branch-and-bound implementation, a basic runway sequence visualisation tool, and a dataset generator.

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

However, a rolling horizon extension to the algorithm has not yet been implemented. The time taken to produce a basic working branch-and-bound implementation without de-icing was longer than expected -- the original plan allocated approximately two weeks for this, but in reality it required closer to three weeks. This was primarily due to issues with adapting a classic branch-and-bound method to utilizing the preprocessed sets of separation-identical aircraft, and underestimation of the workload of other modules.

Despite this, the project is on schedule since the original plan had accounted for such delays -- the original Gantt chart includes gaps of multiple days at various points througout the timeline. Certain tasks -- such as developing an initial visualisation tool -- took much less time than expected, further offsetting the delay incurred by the branch-and-bound-implementation. Additionally, the tasks that have been completed have laid down most of the groundwork for the tasks yet to come, and have improved my overall understanding of and grasp on the problem domain. 

Based on this, the timelines for some remaining tasks have been revised, and Gantt chart has been updated -- see @revised-gantt -- to reflect the actual tasks completed during the first half of the project, as well as the remaining (revised) tasks. The second half of the project will focus mainly on applying more optimisations and pruning rules to the existing branch-and-bound algorithm, implementing a mathematical programming algorithm, and implementing a dymamic programming algorithm.

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

#figure(revised-gantt, caption: [Revised Gantt chart], gap: 1em) <revised-gantt>

== Project Management

The project was managed in an Agile way with weekly (and sometimes bi-weekly) sprints, each having clear tasks to complete. Weekly meetings were held to discuss the project's progress, plan the tasks for the current week, and clarify any issues encountered. These were very effective at ensuring a constant flow of development without long breaks, and enabled the discovery and discussion of problems relatively early.

== Reflection

Reflecting upon the project this term, I believe that I have made good progress towards realising the overall goals of the project. Although there were a few delays in completing the tasks set out in the initial project plan, I have been able to complete the vast majority of them, and have set up a clear direction and foundation for the remainder. This should reduce the overall time required for some of the tasks in the next semester, which is also reflected by the Gantt chart in @revised-gantt.

In its current state, the project already consists of a branch-and-bound algorithm that is capable of solving the integrated runway and de-icing sequencing problem. Next semester, I plan to extend this with a rolling horizon, apply some of the optimisations proposed in existing literature, and further analyse its performance and look for inherent characteristics of the problem that can be exploited to improve the algorithm. I will also implement the other approaches -- mathematical programming and dynamic programming, and undertake a detailed analysis and comparison of their performance.

// TODO: Talk about LSEPI
#todo("Talk about relation to LSEPI")

= References

#bibliography("references.yml", title: none)