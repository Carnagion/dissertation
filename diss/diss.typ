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
#set table(align: center + horizon, inset: 6.5pt, stroke: none)
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

== Approximate Methods

#todo("Write short introduction to approximate methods")

=== Heuristics

#todo("Write about heuristic-based approaches")

== Exact Methods

#todo("Write short introduction to exact methods")

=== Mathematical Programming

#todo("Write about linear and mixed-integer programming approaches used in the past")

=== Dynamic Programming

#todo("Write about dynamic programs used in the past")

== Paradigms

#todo("Write short introduction to paradigms to improve tractability")

=== Constrained Position Shifts

#todo("Write about CPS")

=== Pruning Rules

#todo("Write about pruning rules")

=== Rolling Horizons

#todo("Write about rolling horizon approaches")

= Problem Description

Given a set of arrivals $A$ and departures $D$, the runway and de-icing sequencing problem for a single runway and single de-icing pad consists of finding a sequence of landing and take-off times as well as a sequence of de-icing times such that an optimal value is achieved for a given objective function, subject to the satisfaction of all hard constraints.

== Notation

@notation provides an overview of the symbols used in the following sections along with their definitions.

// TODO: Complete notation table
#let notation = table(
    columns: 2,
    stroke: (x, y) => (
        top: if y == 1 { (dash: "solid", thickness: 0.5pt) },
        left: if x == 1 { (dash: "solid", thickness: 0.5pt) },
    ),
    align: (x, y) => if x > 0 and y > 0 { left + horizon } else { center + horizon },
    table.header[Symbol][Definition],
    $A$, [Set of arrivals],
    $D$, [Set of departures],
    $F$, [Set of aircraft, defined as $A union D$],
    $b_i$, [Base landing or take-off time for aircraft $i$],
    $p_i$, [Pushback duration for departure $i$],
    $m_i$, [Duration to taxi from gates to de-icing pad for departure $i$],
    $o_i$, [De-icing duration for departure $i$],
    $n_i$, [Taxi-out duration for departure $i$],
    $q_i$, [Lineup duration for departure $i$],
    $h_i$, [Maximum holdover duration for departure $i$],
    $r_i$, [Maximum runway hold duration for departure $i$],
    $u_i$, [Start of CTOT slot for departure $i$],
    $v_i$, [End of CTOT slot for departure $i$],
    $e_i$, [Start of hard time window for aircraft $i$],
    $l_i$, [End of hard time window for aircraft $i$],
    $delta_(i, j)$, [Minimum separation between aircraft $i$ and $j$, where $i$ precedes $j$],
    $T_i$, [Set of possible landing or take-off times for aircraft $i$],
    $Z_i$, [Set of possible de-icing times for departure $i$],
    $tau_(i, t)$, [Boolean decision variable indicating if an aircraft $i$ is scheduled to land or take off at time $t$],
    $zeta_(i, z)$, [Boolean decision variable indicating if a departure $i$ is scheduled to de-ice at time $z$],
    $gamma_(i, j)$, [Boolean decision variable indicating whether an aircraft $i$ lands or takes off before an aircraft $j$],
    $t_i$, [Scheduled landing or take-off time for aircraft $i$],
    $z_i$, [Scheduled de-ice time for departure $i$],
    $f(s)$, [Objective value for partial or final sequence $s$],
    $c_d (i)$, [Delay cost for aircraft $i$],
    $c_v (i)$, [CTOT violation cost for departure $i$],
    $F_S$, [Set of pairs of distinct aircraft $(i, j)$ with disjoint hard time windows such that $e_j >= l_i + delta_(i, j)$],
    $F_D$, [Set of pairs of distinct aircraft $(i, j)$ with disjoint hard time windows such that $e_j < l_i + delta_(i, j)$],
    $F_O$, [Set of pairs of distinct aircraft $(i, j)$ with overlapping hard time windows],
    $F_C$, [Set of pairs of distinct separation-identical aircraft $(i, j)$ with a complete order such that $i$ lands or takes off before $j$],
)

#figure(
    notation,
    caption: [Overview of notation and definitions used in model],
) <notation>

== Constraints

A feasible solution to the problem must satisfy runway separation requirements, hard time windows, CTOT slots, holdover times, and runway hold times.
A sequence that violates these hard constraints is considered to be infeasible, and can thus be eliminated from the solution space.

=== Runway Separations

Any two consecutive aircraft $i$ and $j$ (where $i$ precedes $j$) are required to have a minimum runway separation $delta_(i, j)$ between them, which is determined by their weight classes, speed groups, and (for departures) Standard Instrument Departure (SID) routes.
An aircraft's weight class influences the severity of wake turbulence it causes, the time required for this turbulence to dissipate, and its sensitivity to the wake turbulence caused by other aircraft.
Larger or heavier aircraft typically generate greater turbulence, to which smaller or lighter aircraft are more sensitive.
Consequently, a larger separation may be required when a large aircraft is followed by a small one, than when a small aircraft is followed by a large one @demaere-pruning-rules.

// TODO: Check if we actually need to mention this or can leave it out or shorten it
Similarly, a larger separation may be required when a slow aircraft is followed by a faster one on the same route, to prevent the latter from catching up to the former before their routes diverge.
Separations for SID routes are also influenced by the climb and relative bearing of the route, as well as congestion in downstream airspace sectors.
The latter factor may require an increased separation upon take-off to space out traffic and prevent the overloading of en-route sectors and controllers @demaere-pruning-rules.

// TODO: Check if successive vs complete separations and the triangle inequality should be mentioned
The minimum separation that must be maintained between two aircraft is thus the maximum of the separations due to their weight classes, speed groups, and SID routes.
The required separations between each ordered pair of distinct aircraft can therefore be expressed as a separation matrix @demaere-pruning-rules.

However, runway separations do not necessarily obey the triangle inequality -- i.e. for any three aircraft $i$, $j$, and $k$, the inequality $delta_(i, j) + delta_(j, k) >= delta_(i, k)$ is not necessarily true @demaere-pruning-rules.
An aircraft's landing or take-off time can thus be influenced by not just the immediately preceding aircraft, but by multiple preceding aircraft.

=== Time Windows

If an aircraft $i$ is subject to a hard time window which is defined by its earliest (start) time $e_i$ and latest (end) time $l_i$, then its landing or take-off time $t_i$ must be within this window -- i.e. $e_i <= t_i <= l_i$.

In this model, each aircraft is assumed to be subject to a hard time window, although this is not always the case in the real world.
However, this assumption can be made without loss of generality -- an aircraft $i$ that is not subject to a hard time window can instead be considered to be subject to a very large time window, such that its start time $e_i$ is early enough and its end time $l_i$ late enough so as to never affect solutions in practice @demaere-pruning-rules.

=== Calculated Take-Off Times

In addition to a hard time window, a departure $i$ might be subject to a Calculated Take-Off Time (CTOT) slot, during which it should take off.
Typically, a CTOT has a tolerance of -5 to +10 minutes (i.e. five minutes before and ten minutes after $c_i$) and its time window can thus be defined by its earliest (start) time $u_i$ and latest (end) time $v_i$; however, this model makes no such assumptions and allows for customizable CTOT tolerances per departure.

Much like a hard time window, a departure cannot take off before $u_i$, but it may be scheduled after $v_i$ -- although this is heavily penalized.
The start time of a CTOT slot is thus modeled as a hard constraint, while its end time is modeled as a soft constraint.

=== Holdover Times

Once a departure $i$ has been de-iced, the applied de-icing fluid will remain effective for a certain duration of time, called the Holdover Time (HOT) $h_i$.
Departures must take off within this period of time -- if a departure's HOT expires before it takes off, it must be de-iced again, which could extend the de-icing queue and delay subsequent aircraft.

The HOT of a departure $i$ is thus modeled as a hard constraint -- the time between its de-ice time $z_i$ and take-off time $t_i$ must not be greater than $h_i$.

=== Runway Hold Times

// TODO: Write a better explanation for this section
Delays are ideally absorbed by stand holding -- a departure $i$ only needs to push back only when absolutely necessary to meet its de-ice time $z_i$ (if applicable) and take-off time $t_i$.

However, in some cases it may be better to absorb delays at the runway instead by runway holding -- i.e. arriving and waiting at the runway before a departure's scheduled take-off time.
A departure that pushes back earlier than absoltuely necessary would be able to de-ice earlier than necessary, freeing up the de-icing queue earlier.
This could in turn enable the following departures to de-ice earlier and potentially reduce the total delay and CTOT violations in the remaining sequence.

The maximum runway holding duration $r_i$ for a departure $i$ is thus modeled as a hard constraint -- the time between $z_i$ and $t_i$ must not be greater than the sum of its de-ice duration $o_i$, post de-ice taxi duration $n_i$, lineup duration $q_i$, and maximum runway holding duration $r_i$.
That is, $t_i - z_i <= o_i + n_i + q_i + r_i$.

== Objectives

The objective function $f(s)$ for a partial or final sequence $s$ is defined in @objective-function.
It considers total delay and CTOT compliance, and is based on the function described by #cite(<demaere-pruning-rules>, form: "prose").

// TODO: Check if this looks better when mentioned elsewhere, like in the branch-and-bound section
=== Runway Utilization

The runway utilization of a partial or final sequence $s$ is modeled as the makespan of $s$, i.e. $max_(i in s) t_i$.
Although not directly included as an objective, it is utilized for the evaluation of partial sequences generated by the branch-and-bound program and their subsequent pruning according to the pruning rules introduced by #cite(<demaere-pruning-rules>, form: "prose").

=== Delay

// TODO: Maybe word this better
The delay for an aircraft $i$ is defined as the difference between its landing or take-off time $t_i$ and its base time $b_i$.
Its delay cost $c_d (i)$, defined in @delay-cost, is then calculated as the delay squared, and is equivalent to the following function:

$
c_d (i) = (t_i - b_i)^2
$

Raising the delay cost to a power greater than one penalizes disproportionately large delays more severely and encourages a more equitable distribution of delay across all aircraft @demaere-pruning-rules.
For instance, two aircraft with delays of one and three minutes each would have a total delay cost of $1^2 + 3^2 = 10$, whereas the same two aircraft with delays of two minutes each would have a total delay cost of only $2^2 + 2^2 = 8$, making the latter more preferable.

=== Calculated Take-Off Time Compliance

// TODO: Maybe word this better
The CTOT violation cost $c_v (i)$ for a departure $i$ is defined in @ctot-violation-cost, and is equivalent to the following piecewise non-linear function given by 0 if it takes off within its CTOT slot and the squared difference between its takeoff time $t_i$ and its CTOT slot end time $v_i$ if it misses its CTOT slot:

$
c_v (i) = cases(
    &0 &"if" &u_i <= t_i <= v_i,
    &(t_i - v_i)^2 &"if" &t_i > v_i,
)
$

== Model

Based on the constraints and objectives discussed above, a 0-1 integer linear model of the runway and de-icing sequencing problem for a single runway and single de-icing pad is presented below:

// NOTE: Hack for fine-grained equation numbering - see https://github.com/typst/typst/issues/380 and https://github.com/typst/typst/issues/380#issuecomment-1523884719
#let multi-equation(equations) = {
    let reduce(array, f) = array.slice(1).fold(array.first(), f)
    let concat(array) = reduce(array, (acc, elem) => acc + elem)

    if equations.has("children") {
        let children = equations.children.filter(child => child != [ ])

        let body-or-children(equation) = if equation.body.has("children") {
            concat(equation.body.children)
        } else {
            equation.body
        }

        let hide-equation(equation) = if equation.has("numbering") and equation.numbering == none {
            math.equation(block: true, numbering: none, hide(equation))
        } else [
            $ #hide(body-or-children(equation)) $ #if equation.has("label") { equation.label }
        ]

        let hidden = box(concat(children.map(hide-equation)))

        let align-equations(acc, equation) = acc + if acc != [] { linebreak() } + equation
        let aligned = math.equation(block: true, numbering: none, children.fold([], align-equations))

        // NOTE: Spacing needs to be explicitly set to exactly this value for the hack to work
        show math.equation: set block(spacing: 0.65em)

        hidden
        style(style => v(-measure(hidden, style).height, weak: true))
        aligned
    }
}

#let demath = text.with(font: "EB Garamond", weight: "regular")

#multi-equation[
    $ "Minimize" space &f(s) = sum_(i in s) c_d (i) + c_v (i) $ <objective-function>
    $ &c_d (i) = sum_(t in T_i) tau_(i, t) dot (t - b_i)^2 &forall i in F $ <delay-cost>
    $ &c_v (i) = sum_(t in T_i) tau_(i, t) dot (t > v_i) dot (t - v_i)^2 &forall i in D $ <ctot-violation-cost>
    $ &t_i = sum_(t in T_i) tau_(i, t) dot t &forall i in F $ <scheduled-time>
    $ &z_i = sum_(z in Z_i) zeta_(i, z) dot z &forall i in D $ <deice-time>
    $ "Subject to" space &sum_(t in T_i) tau_(i, t) = 1 &forall i in F $ <schedule-once>
    $ &sum_(z in Z_i) zeta_(i, z) = 1 &forall i in D $ <deice-once>
    $ &gamma_(i, j) + gamma_(j, i) = 1 &forall i in F, j in F, i != j $ <schedule-precedence>
    $ &z_j >= z_i + o_i or z_i >= z_j + o_j &forall i in D, j in D, i != j $ <deice-precedence>
    $ &t_i >= z_i + o_i + n_i + q_i &forall i in D $ <min-taxi>
    $ &t_i - z_i - o_i <= h_i &forall i in D $ <max-holdover>
    $ &t_i - z_i - o_i <= n_i + r_i + q_i &forall i in D $ <max-runway-hold>
    $ &gamma_(i, j) = 1 &forall (i, j) in F_S union F_D union F_C $ <certain-precedence>
    $ &t_j >= t_i + delta_(i, j) &forall (i, j) in F_D union F_C $ <complete-order-separation>
    $ &t_j >= t_i + delta_(i, j) dot gamma_(i, j) - (l_i - e_j) dot gamma_(j, i) &forall (i, j) in F_O $ <overlapping-window-separation>
    $ &tau_(i, t) in { 0, 1 } &forall i in F, t in T_i $ <schedule-binary>
    $ &zeta_(i, z) in { 0, 1 } &forall i in D, z in Z_i $ <deice-binary>
    $ &gamma_(i, j) in { 0, 1 } &forall i in F, j in F, i != j $ <precedence-binary>
]

// TODO: Improve wording of this section if necessary

// NOTE: The supplement of these references are customised in a way to group multiple references into one
//       (i.e. "Constraints 1, 2, and 3" instead of "Constraint 1, Constraint 2, and Constraint 3"). This
//       is a hack, but Typst currently has no native or external solutions (package) for this.

The objective function used in the model (@objective-function) minimises total delay and CTOT violations, whose individual costs are given by @delay-cost and @ctot-violation-cost respectively.

@scheduled-time[Constraints] and @deice-time[] define the scheduled landing or take-off time and the de-ice time (if applicable) for an aircraft.

@schedule-once[Constraints] and @deice-once[] ensure that every aircraft is scheduled to land or take off and de-ice exactly once, and within its time window.

@schedule-precedence[Constraint] enforces precedence constraints for every aircraft -- either $i$ must land (or take off) before $j$, or the other way around.

@deice-precedence[Constraint] enforces de-icing precedence constraints for every departure, and ensures that a departure can only begin de-icing after the current aircraft (if any) has finished being de-iced.

@min-taxi[Constraint] ensures that a departure has enough time to taxi out after it finishes de-icing and lineup at the runway to meet its scheduled take-off time.

@max-holdover[Constraint] ensures that the time between a departure's scheduled take-off time and de-ice time does not exceed its allowed HOT -- i.e. once de-iced, departures take off before their HOT expires.

@max-runway-hold[Constraint] ensures that the runway holding time of a departure does not exceed its maximum allowed runway holding time.

@certain-precedence[Constraints], @complete-order-separation[], and @overlapping-window-separation[] enforce precedence and minimum separation constraints on all pairs of distinct aircraft.
These constraints are inferred from disjoint time windows, overlapping time windows, and separation-identical sets of aircraft, and are further discussed in @disjoint-windows and @complete-orders respectively.

@schedule-binary[Constraints], @deice-binary[], and @precedence-binary[] restrict the decision variables for landings or take-offs, de-icing, and aircraft precedences to binary values.

=== Time-Indexed Formulations

#todo("Explain time-indexed formulations")

=== Disjoint Time Windows <disjoint-windows>

#todo("Explain disjoint time windows")

=== Complete Orders <complete-orders>

#todo("Explain complete orders in separation-identical aircraft")

// TODO: Check if pruning rules such as complete orders and disjoint time windows should be mentioned here
= Implementation

#todo("Write short introduction to different approaches used")

== Branch-and-Bound Program

#todo("Write short introduction to branch-and-bound program and various de-icing strategies")

=== Decomposed De-Icing

#todo("Include explanation and pseudocode for decomposed de-icing by TOBT as well as by CTOT")

=== Integrated De-Icing

#todo("Include explanation and pseudocode for integrated de-icing")

=== Rolling Horizon Extension

#todo("Include explanation and pseudocode for rolling horizon")

== Mathematical Program

#todo("Write about CPLEX model")

// TODO: Check if this belongs here or is better off somewhere else
== Sequence Visualiser

#todo("Write about visualiser")

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
    csv("results/heathrow/instances-small.csv"),
    csv("results/heathrow/instances-medium.csv"),
    csv("results/heathrow/instances-large.csv"),
)

#figure(
    heathrow-instances,
    caption: [
        Overview of problem instances from London Heathrow
    ],
) <heathrow-instances>

#todo("Write more about problem instances if necessary")

== Comparison of De-Icing Strategies

#todo("Write about comparison of CPLEX model as well as branch-and-bound, with different de-icing strategies")

@branch-bound-heathrow-small-medium lists the results for the branch-and-bound program using each de-icing approach on all small- and medium-sized problem Heathrow instances.
The small problem instances were solved without a rolling horizon, while a rolling horizon of 12 was used for the medium-sized instances.

#let branch-bound-heathrow-small-medium = results-table(
    group-headers: ([Decomposed de-icing (by TOBT)], [Decomposed de-icing (by CTOT)], [Integrated de-icing]),
    side-headers: true,
    csv("results/heathrow/branch-bound/deice-tobt.csv").slice(0, 20 + 1),
    csv("results/heathrow/branch-bound/deice-ctot.csv").slice(0, 20 + 1),
    csv("results/heathrow/branch-bound/deice-integrated.csv").slice(0, 20 + 1),
)

#align(
    center,
    rotate(-90deg, reflow: true)[   
        #figure(
            branch-bound-heathrow-small-medium,
            caption: [
                Results for small and medium problem instances from London Heathrow solved by the branch-and-bound program utilising the different de-icing strategies
            ],
        ) <branch-bound-heathrow-small-medium>
    ],
)

@branch-bound-furini lists the results for the branch-and-bound program using each de-icing approach on all benchmark instances introduced by #cite(<furini-improved-horizon>, form: "prose").
Since these instances do not contain de-icing data, the pushback duration $p_i$, taxi (to de-icing pads) duration $m_i$, de-icing duration $o_i$, taxi-out duration $n_i$, and lineup duration $q_i$ are assumed to be five minutes each for all departures.
A rolling horizon of size 12 was used to solve each instance.

#let branch-bound-furini = results-table(
    group-headers: ([Decomposed de-icing], [Integrated de-icing]),
    side-headers: true,
    csv("results/furini/branch-bound/deice-decomposed.csv"),
    csv("results/furini/branch-bound/deice-integrated.csv"),
)

#figure(
    branch-bound-furini,
    caption: [
        Results for the benchmark problem instances introduced by #cite(<furini-improved-horizon>, form: "prose") solved by the branch-and-bound program utilising the different de-icing strategies
    ],
) <branch-bound-furini>

As evidenced by the runtimes in @branch-bound-furini, #cite(<furini-improved-horizon>, form: "prose") instances are considerably easier to solve than the Heathrow instances, despite having more aircraft per instance (including more departures to de-ice) and using a larger rolling horizon.
This is primarily due to the lack of CTOT slots as well as the presence of relatively simple separation matrices, which allows complete orders to be inferred between most aircraft in each instance.

// TODO: Check the accuracy of the numbers here
The decomposed de-icing strategy failed to produce feasible solutions for instances FPT01 and FPT06 past 35 aircraft.
A rolling horizon of 20 or higher is required to solve these instances using decomposed de-icing; however, the resulting objective values are still worse than those obtained by the integrated strategy using a lower rolling horizon of 12.

== Comparison of Programs

#let cplex-branch-bound-heathrow-small = results-table(
    group-headers: ([CPLEX model], [Branch-and-bound program]),
    side-headers: true,
    csv("results/heathrow/cplex/deice-integrated.csv"),
    csv("results/heathrow/branch-bound/deice-integrated.csv").slice(0, 10 + 1),
)

#figure(
    cplex-branch-bound-heathrow-small,
    caption: [
        Results for small problem instances from London Heathrow solved by CPLEX as well as a branch-and-bound program, both utilising integrated de-icing
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

// TODO

// sort by earliest de-icing time
// then sort such that the ones with CTOTs jump in front of the others
// in this second pass build up the deicing time
// if you meet a CTOT airrcraft that when deiced in its current position would miss its CTOT then move it before
// almost like a bubble sort

// boxplot for runtimes

// have separate subsections for de-icing strategies and CPLEX vs bnb