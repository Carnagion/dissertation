#import "@preview/lovelace:0.2.0": *
#import "@preview/timeliney:0.0.1": *

// NOTE: Needs to be called at the top of the document to setup lovelace
#show: setup-lovelace

#set text(font: "EB Garamond", size: 11pt)
#set par(justify: true)

#set heading(numbering: "1.1")
#show heading: set block(above: 2em, below: 1.3em)

#set math.equation(numbering: "(1)")

// NOTE: Workaround to get non-math text to use EB Garamond in math equations until Typst ships a native function for doing so
#let mathtext = math.text.with(font: "EB Garamond", weight: "regular")

#let pseudocode = pseudocode.with(indentation-guide-stroke: 0.1pt)

#set figure(gap: 1em)
#show figure.caption: caption => {
    set text(size: 10pt)
    strong(caption.supplement)
    [ ]
    context strong(caption.counter.display(caption.numbering))
    [: ]
    caption.body
}

#show figure.where(kind: table): set block(breakable: true)
#show figure.where(kind: table): set par(justify: false)

// TODO: Pick a good table style
#set table(inset: 6.5pt, stroke: none)
#set table.cell(align: center + horizon)
#show table.cell.where(y: 0, rowspan: 1): strong
#show table.cell: set text(size: 10pt)
#set table.header(repeat: false)

// NOTE: Workaround to make prose citations use "et al" with a lower author count threshold
// TODO: Check if there is a way to already do this in Typst without using a CSL file
#show cite.where(form: "prose"): set cite(style: "ieee-et-al-min.csl")

// TODO: Remove when all todos are removed
#let todo(message) = raw("// TODO: " + message, block: true, lang: "rust")

#v(1fr)
#align(center)[
    // TODO: Should the heading be changed to something better?
    // TODO: Experiment with a proper heading instead of larger text size
    #text(size: 18pt)[*Integrated Aircraft Runway Sequencing and De-Icing*]

    // TODO: Check if "Final Dissertation" is what it should be called
    #text(size: 14pt)[_COMP3003 Final Dissertation_]

    #v(0.2fr)

    #let email(email) = link("mailto:" + email, raw(email))

    // TODO: Check if name and supervisor details should be included
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

#pagebreak()

#v(1fr)

#align(center)[
    // TODO: Experiment with a proper heading instead of larger text size
    #text(size: 14pt)[*Abstract*]
]

#todo("Write abstract")

#lorem(100)

#lorem(50)

#v(1fr)

#pagebreak()

#outline(indent: auto)
#pagebreak()

// NOTE: Done after cover page, abstract, and table of contents since we don't want page numbers to show up on them
// TODO: Decide if previous pages should have numbering as well and if they should have a different style of numbering
#set page(numbering: "1")

// NOTE: Forces the page numbering to begin from here rather than from the cover page or abstract page
// TODO: Decide if numbering should begin from previous pages but should not be shown
#counter(page).update(1)

// TODO: Revise all headings as necessary

= Introduction

#todo("Write introduction")

== Background

#todo("Write background")

== Motivation

#todo("Write about motivation")

= Existing Literature

#todo("Write short introduction to existing literature and past approaches")

== Heuristic-Based Approaches

#todo("Write about heuristic-based approaches used in the past")

== Linear Programming

#todo("Write about linear and mixed-integer programming approaches used in the past")

== Dynamic Programming

#todo("Write about dynamic programs used in the past")

== Constrained Position Shifts

#todo("Write about CPS")

== Pruning Rules

#todo("Write about pruning rules")

= Problem Description

Given a set of arrivals $A$ and departures $D$, the runway and de-icing sequencing problem for a single runway and de-icing pad consists of finding a sequence of landing and take-off times as well as a sequence of de-icing times such that an optimal value is achieved for a given objective function, subject to the satisfaction of all hard constraints.

== Constraints

A feasible solution to the problem must satisfy runway separation requirements, hard time windows, CTOT slots, and holdover times.
A sequence that violates these hard constraints is considered to be infeasible, and can thus be eliminated from the solution space.

=== Runway Separations

Any two consecutive aircraft $i$ and $j$ (where $i$ precedes $j$) are required to have a minimum runway separation $delta_(i j)$ between them, which is determined by their weight classes, speed groups, and (for departures) Standard Instrument Departure (SID) routes.
An aircraft's weight class influences the severity of wake turbulence it causes, the time required for this turbulence to dissipate, and its sensitivity to the wake turbulence caused by other aircraft.
Larger or heavier aircraft typically generate greater turbulence, to which smaller or lighter aircraft are more sensitive.
Consequently, a larger separation may be required when a large aircraft is followed by a small one, than when a small aircraft is followed by a large one @demaere-pruning-rules.

// TODO: Check if we actually need to mention this or can leave it out or shorten it
Similarly, a larger separation may be required when a slow aircraft is followed by a faster one on the same route, to prevent the latter from catching up to the former before their routes diverge.
Separations for SID routes are also influenced by the climb and relative bearing of the route, as well as congestion in downstream airspace sectors.
The latter factor may require an increased separation upon take-off to space out traffic and prevent the overloading of en-route sectors and controllers @demaere-pruning-rules.

// TODO: Check if successive vs complete separations from Beasley's and Geert's papers should be mentioned
The minimum separation that must be maintained between two aircraft is thus the maximum of the separations due to their weight classes, speed groups, and SID routes.
The required separations between each ordered pair of distinct aircraft can thus be expressed as a separation matrix @demaere-pruning-rules.

=== Time Windows

If an aircraft $i$ is subject to a hard time window $T_i$ which is defined by its earliest (start) time $e_i$ and latest (end) time $l_i$, then its landing (or take-off) time $t_i$ must be within this window.
In other words, $e_i <= t_i <= l_i$.

In this model, each aircraft is assumed to be subject to a hard time window, although this is not always the case in the real world.
However, this assumption can be made without loss of generality -- an aircraft $i$ that is not subject to a hard time window can instead be considered to be subject to a very large time window, such that its start time $e_i$ is early enough and its end time $l_i$ late enough so as to never affect solutions in practice @demaere-pruning-rules.

=== Calculated Take-Off Times

In addition to a hard time window, a departure $i$ might be subject to a Calculated Take-Off Time (CTOT) slot $C_i$, during which it should take off.
Typically, a CTOT has a tolerance of -5 to +10 minutes (i.e. five minutes before and ten minutes after $c_i$) and its time window can thus be defined by its earliest (start) time $u_i$ and latest (end) time $v_i$; however, this model makes no such assumptions and allows for customizable CTOT tolerances per departure.

Much like a hard time window, a departure cannot take off before $u_i$, but it may be scheduled after $v_i$ -- although this is heavily penalized.
This is discussed in detail in @ctot-compliance.
The start time of a CTOT slot is thus modeled as a hard constraint, while its end time is modeled as a soft constraint.

=== Holdover Times

Once a departure $i$ has been de-iced, the applied de-icing fluid will remain effective for a certain duration of time, called the Holdover Time (HOT) $h_i$.
Departures must take off within this period of time -- if a departure's HOT expires before it takes off, it must be de-iced again, which could extend the de-icing queue and delay the take-off times of future aircraft.
HOTs are thus modeled as hard constraints.

== Objectives

The objective function $F(s)$ used for this problem is defined in @objective-function.
It considers overall runway utilization (makespan), delay, CTOT compliance, and stand holding time, and is partially based on the function described by #cite(<demaere-pruning-rules>, form: "prose"). 

$
F(s) = (max_(i in s) t_i, sum_(i in s) (B(i) + V(i) + G(i)))
$ <objective-function>

=== Runway Utilization

The runway utilization of a partial or final sequence $s$ is modeled as the makespan of $s$, i.e. $max_(i in s) t_i$.
This is utilized for the evaluation of partial sequences generated by the branch-and-bound program and their subsequent pruning according to the pruning rules introduced by #cite(<demaere-pruning-rules>, form: "prose").

=== Delay

The delay for an aircraft $i$ is defined as the difference between its landing or take-off time $t_i$ and its base time $b_i$.
Its delay cost $B(i)$ is then calculated as the delay squared:

$
B(i) = (t_i - b_i)^2
$

Squaring the delay penalizes disproportionately large delays more severely and encourages a more equitable distribution of delay across all aircraft.

=== Calculated Take-Off Time Compliance <ctot-compliance>

The CTOT violation cost $V(i)$ for a departure $i$ is a piecewise non-linear function given by 0 if it takes off within its CTOT slot and the squared difference between its takeoff time $t_i$ and its CTOT slot end time $v_i$ if it misses its CTOT slot:

$
V(i) = cases(
    &0 &"if" &e_i <= t_i <= v_i,
    &(t_i - v_i)^2 &"if" &t_i > v_i,
)
$

// TODO: Check if this should be in the constraints section instead
// TODO: Explain this more succinctly, decide on notation, and add equations
=== Stand Holding

A departure $i$ would ideally start pushing back such that it exactly meets its de-ice time $d_i$ (if applicable) and take-off time $t_i$.
This forces delays to be absorbed at the stand rather than at the runways, minimising fuel consumption.

However, in some cases, pushing back earlier than necessary would enable an aircraft to de-ice earlier than necessary, freeing up the de-icing queue earlier than if it had pushed back to meet $t_i$ exactly.
This would in turn enable the following departures to de-ice earlier, potentially reducing the total delay and CTOT violations in the sequence.

// TODO: Check if pruning rules such as complete orders and disjoint time windows should be mentioned here
= Implementation

#todo("Write short introduction to different approaches used")

== Model

#todo("Include final mathematical model, objectives, and constraints")

== Branch-and-Bound Program

#todo("Write short introduction to branch-and-bound program and various de-icing strategies")

=== Decomposed De-Icing

#todo("Include explanation and pseudocode for decomposed de-icing by TOBT as well as by CTOT")

=== Integrated De-Icing

#todo("Include explanation and pseudocode for integrated de-icing")

=== Rolling Horizon Extension

#todo("Include explanation and pseudocode for rolling horizon")

// TODO: Check if this belongs here or is better off somewhere else
== Sequence Visualizer

#todo("Write about visualizer")

= Results

#todo("Write introduction to results")

#let results-table(group-headers: (), side-headers: false, ..datasets) = {
    let header-groups = datasets
        .pos()
        .map(array.first)
    
    let data-groups = datasets
        .pos()
        .map(data-group => data-group.slice(1))

    let side-header = ()
    let side-data = ()
    let data = array.zip(..data-groups).flatten()
    if side-headers {
        side-header = (table.cell(rowspan: 2, header-groups.first().first()),)
        side-data = data-groups.first().map(array.first)

        header-groups = header-groups.map(header-group => header-group.slice(int(side-headers)))
        data-groups = data-groups.map(data-group => data-group.map(row => row.slice(int(side-headers))))

        data = side-data.zip(..data-groups).flatten()
    }

    data = data.map(str.trim).map(table.cell)

    let headers = header-groups
        .flatten()
        .map(str.trim)
        .map(table.cell)

    let group-headers = group-headers
        .zip(header-groups.map(array.len))
        .map(pair => {
            let (group-header, headers-len) = pair
            table.cell(colspan: headers-len, group-header)
        })

    let header-rowspan = if group-headers == () { 1 } else { 2 }
    let header-colspan = header-groups.first().len()
    
    let stroke = (x, y) => {
        let stroke-style = (dash: "solid", thickness: 0.5pt)
        let h = if y > 0 and y <= header-rowspan {
            stroke-style
        }
        let v = if x != 0 and calc.rem-euclid(x, header-colspan) == side-header.len() {
            stroke-style
        }
        (
            top: h,
            bottom: none,
            left: v,
            right: none,
        )
    }

    table(
        columns: side-header.len() + headers.len(),
        stroke: stroke,
        table.header(
            ..side-header,
            ..group-headers,
            ..headers,
        ),
        ..data,
    )
}

== Problem Instances

// TODO: Check if Heathrow or University of Bologna should be cited
The performance of both the branch-and-bound program and CPLEX model is illustrated here using complex real-world problem instances from a single day of departure operations at London Heathrow -- whose characteristics are summarized in @heathrow-instances -- as well as benchmark problem instances from Milan Airport. The latter were obtained from the University of Bologna Operations Research Group's freely accessible #link("https://site.unibo.it/operations-research/en/research/library-of-codes-and-instances-1")[online library of instances].

#let heathrow-instances = results-table(
    group-headers: ([Small], [Medium], [Large]),
    csv("results/heathrow-instances-small.csv"),
    csv("results/heathrow-instances-medium.csv"),
    csv("results/heathrow-instances-large.csv"),
)

#figure(
    heathrow-instances,
    caption: [
        Overview of problem instances from London Heathrow
    ],
) <heathrow-instances>

#todo("Write more about problem instances if necessary")

== Comparison of Approaches

#todo("Write about comparison of CPLEX model as well as branch-and-bound, with different de-icing strategies")

@branch-bound-heathrow-small-medium lists the results for the branch-and-bound program using each de-icing approach on all small- and medium-sized problem Heathrow instances.
The small problem instances were solved without a rolling horizon, while a rolling horizon of 12 was used for the medium-sized instances.

#let branch-bound-heathrow-small-medium = results-table(
    group-headers: ([Decomposed de-icing (by TOBT)], [Decomposed de-icing (by CTOT)], [Integrated de-icing]),
    side-headers: true,
    csv("results/branch-bound-tobt-heathrow-small-medium.csv"),
    csv("results/branch-bound-ctot-heathrow-small-medium.csv"),
    csv("results/branch-bound-integrated-heathrow-small-medium.csv"),
)

#align(
    center,
    rotate(-90deg, reflow: true)[   
        #figure(
            branch-bound-heathrow-small-medium,
            caption: [
                Results for small and medium problem instances from London Heathrow solved by a branch-and-bound approach utilising various de-icing strategies
            ],
        ) <branch-bound-heathrow-small-medium>
    ],
)

@branch-bound-furini lists the results for the branch-and-bound program using each de-icing approach on all benchmark instances introduced by #cite(<furini-improved-horizon>, form: "prose").
Each aircraft is assumed to take five minutes for pushback, taxiing, de-icing, and lineup respectively, and a rolling horizon of 12 was used to solve each instance.

#let branch-bound-furini = results-table(
    group-headers: ([Decomposed de-icing], [Integrated de-icing]),
    side-headers: true,
    csv("results/branch-bound-decomposed-furini.csv"),
    csv("results/branch-bound-integrated-furini.csv"),
)

#figure(
    branch-bound-furini,
    caption: [
        Results for benchmark problem instances introduced by #cite(<furini-improved-horizon>, form: "prose") solved by a branch-and-bound approach utilising various de-icing strategies
    ],
) <branch-bound-furini>

As evidenced by the runtimes, #cite(<furini-improved-horizon>, form: "prose") instances are considerably easier to solve than the Heathrow instances, despite having more aircraft per instance and using a larger rolling horizon.
This is primarily due to the lack of CTOT slots as well as the presence of relatively simple separation matrices, which allows complete orders to be inferred between most aircraft in each instance.

#let cplex-branch-bound-heathrow-small = results-table(
    group-headers: ([CPLEX model], [Branch-and-bound program]),
    side-headers: true,
    csv("results/cplex-heathrow-small.csv"),
    csv("results/branch-bound-integrated-heathrow-small.csv"),
)

#figure(
    cplex-branch-bound-heathrow-small,
    caption: [
        Results for small problem instances from London Heathrow solved by CPLEX as well as the branch-and-bound program, both performing integrated de-icing
    ],
) <cplex-branch-bound-heathrow-small>

== Impact

#todo("Write about impact of results")

= Reflections

#todo("Write introduction to reflection")

== Project Management

#todo("Write about project management")

// TODO: Check if a better heading could be used
== Contributions

#todo("Write about LSEPI and contributions")

= References

// NOTE: Title disabled since we want to use a custom title and passing in a heading as the title makes
//       it too big and messes up the table of contents
#bibliography("references.yml", title: none, style: "ieee")