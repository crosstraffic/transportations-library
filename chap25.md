## **CHAPTER 25** **FREEWAY FACILITIES: SUPPLEMENTAL** **CONTENTS**

**1. INTRODUCTION**

Chapter Scope

Chapter Organization

Limitations of the Methodologies


**2. GLOSSARY OF VARIABLE DEFINITIONS**

Overview

Global Variables

Segment Variables

Node Variables

On-Ramp Variables

Off-Ramp Variables

Facilitywide Variables

Travel Time Reliability Variables


**3. UNDERSATURATED SEGMENT EVALUATION**

Facility Speed Constraint

Directional Facility Module


**4. OVERSATURATED SEGMENT EVALUATION**


Procedure Parameters

Flow Estimation

Segment and Ramp Performance Measures

Oversaturation Analysis within Managed Lanes


**5. WORK ZONE ANALYSIS DETAILS**

Special Work Zone Configurations


**6. PLANNING-LEVEL METHODOLOGY FOR FREEWAY**
**FACILITIES**

Input Requirements

Step 1: Demand-Level Calculations

Step 2: Section Capacity Calculations and Adjustments

Step 3: Delay Rate Estimation

Step 4: Average Travel Time, Speed, and Density Calculations

Step 5: Level of Service


**7. MIXED-FLOW MODEL FOR COMPOSITE GRADES**

Overview of the Methodology

Step 1: Input Data

Step 2: Capacity Assessment

Step 3: Specify Initial Conditions

Step 4: Compute Truck Spot and Space-based Travel Time Rates

Step 5: Compute Automobile Spot and Space-Based Travel Time Rates

Step 6: Compute Mixed-Flow Space-Based Travel Time Rate and
Speed

Step 7: Overall Results


**8. FREEWAY CALIBRATION METHODOLOGY**


Calibration at the Core Freeway Facility Level

Calibration at the Travel Time Reliability Level

Calibration at the Reliability Strategy Assessment Level


**9. FREEWAY SCENARIO GENERATION**

Introduction

Methodology


**10. COMPUTATIONAL ENGINE OVERVIEW**


**11. EXAMPLE PROBLEMS**

Example Problem 1: Evaluation of an Undersaturated Facility

Example Problem 2: Evaluation of an Oversaturated Facility

Example Problem 3: Capacity Improvements to an Oversaturated
Facility

Example Problem 4: Evaluation of an Undersaturated Facility with a
Work Zone

Example Problem 5: Evaluation of an Oversaturated Facility with a
Managed Lane

Example Problem 6: Planning-Level Analysis of a Freeway Facility

Example Problem 7: Reliability Evaluation of an Existing Freeway
Facility

Example Problem 8: Reliability Analysis with Geometric Improvements

Example Problem 9: Evaluation of Incident Management

Example Problem 10: Planning-Level Reliability Analysis

Example Problem 11: Estimating Freeway Composite Grade Operations
with the Mixed-Flow Model


**12. REFERENCES**


**APPENDIX A: TRUCK PERFORMANCE CURVES**


## **LIST OF EXHIBITS**

Exhibit 25-1: Node–Segment Representation of a Directional Freeway
Facility

Exhibit 25-2: Segment Flow–Density Function

Exhibit 25-3: Oversaturated Analysis Procedure

Exhibit 25-4: Definitions of Mainline and Segment Flows

Exhibit 25-5: Flow–Density Function with a Shock Wave

Exhibit 25-6: Vertical Queuing from a Managed Lane Due to Queue Presence
on the General Purpose Lanes

Exhibit 25-7: On-Ramp Merge Diagram for 2-to-1 Freeway Work Zone
Configuration

Exhibit 25-8: Proportion of Work Zone Queue Discharge Rate (Relative to
the Basic Work Zone Capacity) Available for Mainline Flow Upstream
of Merge Area

Exhibit 25-9: Off-Ramp Diverge Diagram for a 2-to-1 Freeway Work Zone
Configuration

Exhibit 25-10: Proportion of Work Zone Capacity Available for Mainline
Flow Downstream of Diverge Area

Exhibit 25-11: Proportion of Off-Ramp Demand Served in Work Zone

Exhibit 25-12: Proportion of Available Work Zone Capacity for a
Directional Crossover in the Work Zone

Exhibit 25-13: Model Coefficients for Estimating the Proportion of Work
Zone Capacity in a Weaving Segment

Exhibit 25-14: Model Coefficients for Estimating the Proportion of OffRamp Volume Served in the Weaving Area

Exhibit 25-15: Schematics of Freeway Sections

Exhibit 25-16: Parameter Values for Undersaturated Model

Exhibit 25-17: LOS Criteria for Urban and Rural Freeway Facilities


Exhibit 25-18: Schematic of a Composite Grade

Exhibit 25-19: Mixed-Flow Methodology Overview

Exhibit 25-20: SUT Spot Rates Versus Distance with Initial Speeds of 75 and
30 mi/h

Exhibit 25-21: TT Spot Rates Versus Distance with Initial Speeds of 75 and
20 mi/h

Exhibit 25-22: SUT Travel Time Versus Distance Curves for 70-mi/h Initial
Speed

Exhibit 25-23: SUT Travel Time Versus Distance Curves for 30-mi/h Initial
Speed

Exhibit 25-24: _δ_ Values for SUTs

Exhibit 25-25: _δ_ Values for TTs

Exhibit 25-26: Calibration Steps for the Core Freeway Facility Level

Exhibit 25-27: Effect of Calibrating Free-Flow Speed on Capacity

Exhibit 25-28: Effects of Segment Capacity

Exhibit 25-29: Effects of Queue Discharge Rate Drop

Exhibit 25-30: Effects of Jam Density

Exhibit 25-31: Effect of Demand Level

Exhibit 25-32: Comprehensive Reliability Calibration Steps

Exhibit 25-33: High Demand Level on the Seed Day

Exhibit 25-34: Low Demand Level on the Seed Day

Exhibit 25-35: Overestimating the Impacts of Nonrecurring Sources of
Congestion

Exhibit 25-36: Underestimating the Impacts of Nonrecurring Sources of
Congestion

Exhibit 25-37: Process Flow Overview for Freeway Scenario Generation

Exhibit 25-38: Distribution of Number of Incidents in the Scenarios

Exhibit 25-39: Detailed Freeway Scenario Generation Flowchart


Exhibit 25-40: Listing of Weather Stations with Available Weather Data

Exhibit 25-41: Incident Duration Distribution Parameters in Minutes

Exhibit 25-42: List of Example Problems

Exhibit 25-43: Example Problem 1: Freeway Facility

Exhibit 25-44: Example Problem 1: Geometry of Directional Freeway
Facility

Exhibit 25-45: Example Problem 1: Demand Inputs

Exhibit 25-46: Example Problem 1: Segment Capacities

Exhibit 25-47: Example Problem 1: Segment Demand-to-Capacity Ratios

Exhibit 25-48: Example Problem 1: Volume-Served Matrix

Exhibit 25-49: Example Problem 1: Speed Matrix

Exhibit 25-50: Example Problem 1: Density Matrix

Exhibit 25-51: Example Problem 1: LOS Matrix

Exhibit 25-52: Example Problem 1: Facility Performance Measure Summary

Exhibit 25-53: Example Problem 2: Demand Inputs

Exhibit 25-54: Example Problem 2: Segment Capacities

Exhibit 25-55: Example Problem 2: Segment Demand-to-Capacity Ratios

Exhibit 25-56: Example Problem 2: Volume-Served Matrix

Exhibit 25-57: Example Problem 2: Speed Matrix

Exhibit 25-58: Example Problem 2: Density Matrix

Exhibit 25-59: Example Problem 2: Expanded LOS Matrix

Exhibit 25-60: Example Problem 2: Facility Performance Measure Summary

Exhibit 25-61:Example Problem 3: Freeway Facility

Exhibit 25-62: Example Problem 3: Geometry of Directional Freeway
Facility

Exhibit 25-63: Example Problem 3: Segment Capacities

Exhibit 25-64: Example Problem 3: Segment Demand-to-Capacity Ratios


Exhibit 25-65: Example Problem 3: Speed Matrix

Exhibit 25-66: Example Problem 3: Density Matrix

Exhibit 25-67: Example Problem 3: LOS Matrix

Exhibit 25-68: Example Problem 3: Facility Performance Measure Summary

Exhibit 25-69: Example Problem 4: Freeway Facility

Exhibit 25-70: Example Problem 4: Geometry of Directional Freeway
Facility

Exhibit 25-71: Example Problem 4: Segment Capacities

Exhibit 25-72: Example Problem 4: Segment Demand-to-Capacity Ratios

Exhibit 25-73: Example Problem 4: Volume-Served Matrix

Exhibit 25-74: Example Problem 4: Speed Matrix

Exhibit 25-75: Example Problem 4: Density Matrix

Exhibit 25-76: Example Problem 4: LOS Matrix

Exhibit 25-77: Example Problem 4: Facility Performance Measure Summary

Exhibit 25-78: Example Problem 5: Freeway Facility

Exhibit 25-79: Example Problem 5: Geometry of Directional Freeway
Facility

Exhibit 25-80: Example Problem 5: Demand Inputs on the Mainline

Exhibit 25-81: Example Problem 5: Segment Capacities

Exhibit 25-82: Example Problem 5: Segment Demand-to-Capacity Ratios

Exhibit 25-83: Example Problem 5: Speed Matrix

Exhibit 25-84: Example Problem 5: Density Matrix

Exhibit 25-85: Example Problem 5: LOS Matrix

Exhibit 25-86: Example Problem 5: Facility Performance Measure Summary
for Lane Groups

Exhibit 25-87: Example Problem 5: Facility Performance Measure Summary

Exhibit 25-88: Example Problem 6: AADT Values for the Facility


Exhibit 25-89: Example Problem 6: Section Definition for the Facility

Exhibit 25-90: Example Problem 6: Demand Flow Rates (pc/h) on the
Subject Facility

Exhibit 25-91: Example Problem 6: Demand-to-Capacity Ratios by Section
and Analysis Period

Exhibit 25-92: Example Problem 6: Delay Rates by Section and Analysis
Period

Exhibit 25-93: Example Problem 6: Travel Rates by Section and Analysis
Period

Exhibit 25-94: Example Problem 6: Average Travel Times by Section and
Analysis Period

Exhibit 25-95: Example Problem 6: Density by Section and Analysis Period

Exhibit 25-96: Example Problem 6: Facility Performance Summary

Exhibit 25-97: Example Problem 7: Freeway Facility

Exhibit 25-98: Example Problem 7: Geometry of Directional Freeway
Facility

Exhibit 25-99: Example Problem 7: Demand Flow Rates (veh/h) by Analysis
Period in the Base Data Set

Exhibit 25-100: Example Problem 7: Demand Ratios Relative to AADT

Exhibit 25-101: Example Problem 7: Weather Event Probabilities by Season

Exhibit 25-102: Example Problem 7: CAF, SAF, and Event Duration Values
Associated with Weather Events

Exhibit 25-103: Example Problem 7: Incident Frequencies by Month

Exhibit 25-104: Example Problem 7: Summary Reliability Performance
Measure Results

Exhibit 25-105: Example Problem 7: VMT-Weighted TTI Probability and
Cumulative Distribution Functions

Exhibit 25-106: Example Problem 8: Freeway Facility


Exhibit 25-107: Example Problem 8: Summary Reliability Performance
Measure Results

Exhibit 25-108: Example Problem 9: Summary Reliability Performance
Measure Results

Exhibit 25-109: Example Problem 11: Spot Speeds of All Segments

Exhibit 25-110: Example Problem 11: Space Mean Speeds of All Segments

Exhibit 25-111: Example Problem 11: Overall Space Mean Speeds of All
Segments

Exhibit 25-A1: SUT Travel Time Versus Distance Curves for 35-mi/h Initial
Speed

Exhibit 25-A2: SUT Travel Time Versus Distance Curves for 40-mi/h Initial
Speed

Exhibit 25-A3: SUT Travel Time Versus Distance Curves for 45-mi/h Initial
Speed

Exhibit 25-A4: SUT Travel Time Versus Distance Curves for 50-mi/h Initial
Speed

Exhibit 25-A5: SUT Travel Time Versus Distance Curves for 55-mi/h Initial
Speed

Exhibit 25-A6: SUT Travel Time Versus Distance Curves for 60-mi/h Initial
Speed

Exhibit 25-A7: SUT Travel Time Versus Distance Curves for 65-mi/h Initial
Speed

Exhibit 25-A8: SUT Travel Time Versus Distance Curves for 75-mi/h Initial
Speed

Exhibit 25-A9: TT Travel Time Versus Distance Curves for 20-mi/h Initial
Speed

Exhibit 25-A10: TT Travel Time Versus Distance Curves for 25-mi/h Initial
Speed

Exhibit 25-A11: TT Travel Time Versus Distance Curves for 30-mi/h Initial
Speed


Exhibit 25-A12: TT Travel Time Versus Distance Curves for 35-mi/h Initial
Speed

Exhibit 25-A13: TT Travel Time Versus Distance Curves for 40-mi/h Initial
Speed

Exhibit 25-A14: TT Travel Time Versus Distance Curves for 45-mi/h Initial
Speed

Exhibit 25-A15: TT Travel Time Versus Distance Curves for 50-mi/h Initial
Speed

Exhibit 25-A16: TT Travel Time Versus Distance Curves for 55-mi/h Initial
Speed

Exhibit 25-A17: TT Travel Time Versus Distance Curves for 60-mi/h Initial
Speed

Exhibit 25-A18: TT Travel Time Versus Distance Curves for 65-mi/h Initial
Speed

Exhibit 25-A19: TT Travel Time Versus Distance Curves for 70-mi/h Initial
Speed

Exhibit 25-A20: TT Travel Time Versus Distance Curves for 75-mi/h Initial
Speed


## **1. INTRODUCTION**

**CHAPTER SCOPE**

Chapter 25 is the supplemental chapter for Chapter 10, which describes
the core methodology for freeway facilities, and Chapter 11, which presents
a methodology for evaluating freeway reliability and active traffic and
demand management (ATDM) strategies. The computations used by these
methodologies are detailed in this supplemental chapter. The documentation
is closely tied to FREEVAL-2015E, the computational engine for Chapter 10
and Chapter 11.

The FREEVAL (FREeway EVALuation) tool was initially developed for
the 2000 edition of the _Highway Capacity Manual_ (HCM) ( _1, 2_ ) and has
been updated to reflect subsequent methodological changes in the HCM. All
variable definitions and subroutine labels presented in this chapter are
consistent with the computational code in FREEVAL-2015E. The Technical
Reference Library in Volume 4 contains a FREEVAL-2015E user guide,
which provides more details on how to use the computational engine. Other
software implementations of this method are available and can be used
instead of the computational engine.


**CHAPTER ORGANIZATION**

Section 2 presents a glossary of all relevant variables used in the
procedures and the computational engine. Section 3 and Section 4,
respectively, provide details of the undersaturated and oversaturated flow
procedures. Section 5 describes details for work zone analysis. Section 6
develops the planning-level methodology for freeway facilities, and Section
7 discusses the mixed-flow model for composite grades. Section 8 develops
the freeway calibration methodology at three levels. Section 9 discusses
freeway scenario generation, and Section 10 presents an overview of the
computational engine structure. Example problems are presented in Section
11, and Section 12 provides references for the chapter.


**LIMITATIONS OF THE METHODOLOGIES**

The completeness of the analysis will be limited if freeway segment cells
in the first time interval, the final time interval, and the first freeway segment
do not have demand-to-capacity ratios of 1.00 or less. The methodology can
handle congestion in the first interval properly, although it will not quantify
any congestion that could have occurred before the first time interval. To
ensure a complete quantification of the effects of congestion, it is
recommended that the analysis contain an initial undersaturated time interval.
If all freeway segments in the final time interval do not exhibit demand-tocapacity ratios less than 1.00, congestion will continue beyond the final time
interval, and additional time intervals should be added. This fact will be
noted as a difference between the vehicle miles of travel desired at the end
of the analysis (demand flow) and the corresponding vehicle miles of travel
flow generated (volume served). If queues extend upstream of the first
segment, the analysis will not account for the congestion outside the freeway
facility but will store the vehicles vertically until the congestion clears the
first segment. The same process is followed for queues on on-ramp segments.

The methodology for oversaturated conditions described in this chapter
is based on concepts of traffic flow theory and assumes a linear speed–flow
relationship for densities greater than 45 passenger cars per mile per lane
(pc/mi/ln). This relationship has not been extensively calibrated for field
observations on U.S. freeways, and analysts should therefore perform their
own validation from local data to obtain additional confidence in the results
of this procedure. For an example of a validation exercise for this
methodology, the reader is referred elsewhere ( _3_ ).

The procedure described here becomes extremely complex when the
queue from a downstream bottleneck extends into an upstream bottleneck,
causing a queue interaction. When such cases arise, the reliability of the
methodology is questionable, and the user is cautioned about the validity of
the results. For heavily congested directional freeway facilities with
interacting bottleneck queues, a traffic simulation model might be more
applicable. Noninteracting bottlenecks are addressed by the methodology.

The procedure focuses on analyzing a directional series of freeway
segments. It describes the performance of a facility but falls short of
addressing the broader transportation network. The analyst is cautioned that


severe congestion on a freeway—especially freeway on-ramps—is likely to
affect the adjacent surface street network. Similarly, the procedure is limited
in its ability to predict the impacts of an oversaturated off-ramp and the
associated queues that may spill back onto the freeway. Alternative tools are
suitable to evaluate these impacts.


## **2. GLOSSARY OF VARIABLE DEFINITIONS**

**OVERVIEW**

This glossary defines internal variables used exclusively in the freeway
facilities methodology. The variables are consistent with those used in the
computational engine for the freeway facilities methodology.

If a managed lane facility is adjacent to the general purpose lanes, the
oversaturated freeway facilities methodology will analyze each facility
independently. As a result, the variables presented in this chapter will
pertain to general purpose and managed lane facilities separately.

The glossary of variables is presented in seven parts: global variables,
segment variables, node variables, on-ramp variables, off-ramp variables,
facilitywide variables, and travel time reliability variables. Global variables
are used across multiple aspects of the procedure. Segment variables
represent conditions on segments. Node variables denote flows across a
node connecting two segments. On- and off-ramp variables correspond to
flow on ramps. Facilitywide variables pertain to aggregate traffic
performance over the entire general purpose or managed lane facility.
Reliability variables pertain to traffic performance over a period of up to
one year.

In addition to the spatial categories listed above, there are temporal
divisions that represent characteristics over a time step for oversaturated
conditions or an analysis period for undersaturated conditions. The first
dimension associated with each variable specifies whether the variable
refers to segment or node characteristics. The labeling scheme for nodes and
segments is such that segment _i_ is immediately downstream of node _i_ . The
distinction of nodes and segments is used primarily in the oversaturated flow
regime as discussed in Section 4.

Thus, there is always one more node than the number of segments on a
facility. The second and third dimensions denote a time step _t_ and a time
interval _p_ . Facility variables are estimates of the average performance over


the length of the facility. The units of flow are in vehicles per time step. The
selection of the time step size is discussed later in this chapter.

The variable symbols used internally by the computational engine and
replicated in this chapter frequently differ from the symbols used elsewhere
in the HCM, particularly in Chapter 10, Freeway Facilities Core
Methodology. For example, the HCM uses _n_ to represent the number of
segments forming a facility, whereas the computational engine and this
chapter use _NS_ .


**GLOBAL VARIABLES**

   - _i_ —index to segment or node number: _i_ = 1, 2, …, _NS_ (for segments)
and _i_ = 1, 2, …, _NS_ + 1 (for nodes). In the computational engine, _i_ is
represented as the index of the _GPSegments/MLSegments Array List_
variable in the Seed class.

   - _KC_ —ideal density at capacity (pc/mi/ln). The density at capacity is
45 pc/mi/ln.

   - _KJ_ —facilitywide jam density (pc/mi/ln).

   - _NS_ —number of segments on the facility. _NS_ is represented as the size
of the _GPSegments/MLSegments ArrayList_ variable in the Seed
class.

   - _P_ —number of (15-min) analysis periods in the study period.
Represented as _Period_ in the computational engine. For a 24-h
analysis, the theoretical maximum is 96 analysis periods.

   - _p_ —analysis period index: _p_ = 1, 2, …, _P_ .

   - _S_ —number of computational time steps in an analysis period
(integer). _S_ is represented as _Step_ in the computational engine. _S_ is
set as a constant of 60 in the computational engine, corresponding to a
15-s interval and allowing a minimum segment length of 300 ft.

   - _t_ —time step index in a single analysis period: _t_ = 1, 2, …, _S_ .

   - _T_ —number of time steps in 1 h (integer). _T_ is set as a constant of 240
in the computational engine, or equal to four times the value of _S._


   - _α_ —fraction of capacity drop in queue discharge conditions due to
congestion on the facility. This variable is represented as
_inCapacityDropPercentage_ in the GPMLSegment class in the
computational engine.


**SEGMENT VARIABLES**

   - _ED_ ( _i_, _p_ )—expected demand (veh/h) that would arrive at segment _i_ on
the basis of upstream conditions over time interval _p_ . The upstream
queuing effects include the metering of traffic from an upstream queue
but not the spillback of vehicles from a downstream queue.

   - _K_ ( _i_, _p_ )—average traffic density (veh/mi/ln) of segment _i_ over time
interval _p_ as estimated by the oversaturated procedure. This variable
is represented as the _scenAllDensity_veh_ variable in the
GPMLSegment class in the computational engine.

   - _KB_ ( _i_, _p_ )—background density: segment _i_ density (veh/mi/ln) over
time interval _p_ assuming there is no queuing on the segment. This
density is calculated by using the expected demand on the segment in
the corresponding undersaturated procedure in Chapters 12, 13, and
14.

   - _KQ_ ( _i_, _t_, _p_ )—queue density: vehicle density (veh/mi/ln) in the queue
on segment _i_ during time step _t_ in time interval _p_ . Queue density is
calculated on the basis of a linear density–flow relationship in the
congested regime.

   - _L_ ( _i_ )—length of segment _i_ (mi). This variable converts the
_inSegLength_ft_ variable (in feet) to miles when necessary in
equations.

   - _N_ ( _i_, _p_ )—number of lanes on segment _i_ in time interval _p_ . It could
vary by time interval if a temporary lane closure is in effect. _N_ is
represented as the _inMainlineNumLanes_ variable in the
GPMLSegment class in the computational engine.

   - _NV_ ( _i, p_ )—number of vehicles present on segment _i_ at the end of time
interval _p_ (veh).


   - _NV_ ( _i_, _t_, _p_ )—number of vehicles present on segment _i_ at the end of
time step _t_ during time interval _p_ . The number of vehicles is initially
based on the calculations of Chapters 12, 13, and 14, but, as queues
grow and dissipate, input–output analysis updates these values in
each time step.

   - _Q_ ( _i_, _t_, _p_ )—total queue length on segment _i_ at the end of time step _t_ in
time interval _p_ (ft).

   - _SC_ ( _i_, _t_, _p_ )—segment capacity: maximum number of vehicles that can
pass through segment _i_ at the end of time step _t_ in time interval _p_
based strictly on traffic and geometric properties (veh).

   - _SD_ ( _i_, _p_ )—segment demand: desired flow rate (veh/h) through
segment _i_ including on- and off-ramp demands in time interval _p_
(veh). This segment demand is calculated without any capacity
constraints. It is represented as the _scenMainlineDemand_veh_
variable in the GPMLSegment class in the computational engine.

   - _SF_ ( _i, p_ )—segment flow out of segment _i_ in time interval _p_ (veh/h).

   - _SF_ ( _i_, _t_, _p_ )—segment flow out of segment _i_ during time step _t_ in time
interval _p_ (veh/time step).

   - _U_ ( _i_, _p_ )—average space mean speed over the length of segment _i_
during time interval _p_ (mi/h). It is represented as the _scenSpeed_
variable in the GPMLSegment class in the computational engine.

   - _UV_ ( _i_, _t_, _p_ )—unserved vehicles: the additional number of vehicles
stored on segment _i_ at the end of time step _t_ in time interval _p_ due to a
downstream bottleneck.

   - _WS_ ( _i_, _p_ )—wave speed: speed at which a front-clearing queue shock
wave travels through segment _i_ during time interval _p_ (mi/h).

   - _WTT_ ( _i_, _p_ )—wave travel time: time taken by the shock wave traveling
at wave speed _WS_ to travel from the downstream end of segment _i_ to
the upstream end of the segment during time interval _p_, in time steps.


**NODE VARIABLES**


   - _MF_ ( _i_, _t_, _p_ )—actual mainline flow rate that can cross node _i_ during
time step _t_ in time interval _p_ .

   - _MI_ ( _i_, _t_, _p_ )—maximum mainline input: maximum flow desiring to
enter node _i_ during time step _t_ in time interval _p_, based on flows from
all upstream segments and taking into account all geometric and
traffic constraints upstream of the node, including queues
accumulated from previous time intervals.

   - _MO_ 1( _i_, _t_, _p_ )—maximum Mainline Output 1: maximum allowable
mainline flow rate across node _i_ during time step _t_ in time interval _p_,
limited by the flow from an on-ramp at node _i_ .

   - _MO_ 2( _i_, _t_, _p_ )—maximum Mainline Output 2: maximum allowable
mainline flow rate across node _i_ during time step _t_ in time interval _p_,
limited by available storage on segment _i_ due to a downstream queue.

   - _MO_ 3( _i_, _t_, _p_ )—maximum Mainline Output 3: maximum allowable
mainline flow rate across node _i_ during time step _t_ in time interval _p_,
limited by the presence of queued vehicles at the upstream end of
segment _i_ while the queue clears from the downstream end of segment
_i_ .


**ON-RAMP VARIABLES**

   - _ONRC_ ( _i_, _p_ )—geometric carrying capacity of on-ramp at node _i_
during time interval _p_ .

   - _ONRD_ ( _i_, _p_ )—demand flow rate for on-ramp at node _i_ in time interval
_p_ .

   - _ONRF_ ( _i_, _t_, _p_ )—actual ramp flow rate that can cross on-ramp node _i_
during time step _t_ in time interval _p_ ; it takes into account control
constraints (e.g., ramp meters).

   - _ONRI_ ( _i_, _t_, _p_ )—input flow rate desiring to enter the merge point at onramp _i_ during time step _t_ in time interval _p_, based on current ramp
demand and ramp queues accumulated from previous time intervals.

   - _ONRO_ ( _i_, _t_, _p_ )—maximum output flow rate that can enter the merge
point from on-ramp _i_ during time step _t_ in time interval _p_ ; it is
constrained by Lane 1 (shoulder lane) flow on segment _i_ and the


segment _i_ capacity or by a queue spillback filling the mainline
segment from a bottleneck further downstream, whichever governs.

   - _ONRQ_ ( _i_, _t_, _p_ )—unmet demand that is stored as a queue on the onramp roadway at node _i_ during time step _t_ in time interval _p_ (veh).

   - _RM_ ( _i_, _p_ )—maximum allowable rate of an on-ramp meter at the onramp at node _i_ during time interval _p_ (veh/h).


**OFF-RAMP VARIABLES**

   - _DEF_ ( _i_, _t_, _p_ )—deficit: unmet demand from a previous time interval _p_
that flows past node _i_ during time step _t_ ; it is used in off-ramp flow
calculations downstream of a bottleneck.

   - _OFRD_ ( _i_, _p_ )—desired off-ramp demand flow exiting at off-ramp _i_
during time interval _p_ .

   - _OFRF_ ( _i_, _t_, _p_ )—actual flow that can exit at off-ramp _i_ during time step
_t_ in time interval _p_ .


**FACILITYWIDE VARIABLES**

   - _K_ ( _NS_, _P_ )—average vehicle density over the entire facility during the
entire analysis period _P_ .

   - _K_ ( _NS_, _p_ )—average vehicle density over the entire facility during time
interval _p_ .

   - _SMS_ ( _NS_, _P_ )—average analysis period facility speed: average space
mean speed over the entire facility during the entire analysis period
_P_ .

   - _SMS_ ( _NS_, _p_ )—average time interval facility speed: average space
mean speed over the entire facility during time interval _p_ .


**TRAVEL TIME RELIABILITY VARIABLES**

   - _CRj_ —crash rate per 100 million vehicle miles traveled (VMT) in
month _j_ .

   - _DSP_ —duration of study period _SP_ (h).


- _DAFs_ ( _i_, _p_ )—demand adjustment factor for scenario _s_, time interval _p_,
and segment _i_ .

- _DCs_ —demand combination associated with scenario _s_ .

- _DM_ ( _s_ )—demand multiplier associated with scenario _s_ .

- _DM_ (Seed)—demand multiplier associated with the seed file.


relative to seed value.

- _E_ [ _nw_, _j_ ]—expected frequency of weather event _w_ in month _j_, rounded
to the nearest integer.

- _E_ 15min[ _Dw_ ]—expected duration of weather event _w_, rounded to the
nearest 15-min increment.

- ( _i_ )—distribution function for incident with severity type _i_ .

- _ICR—_ incident-to-crash ratio.

- _IncDur—_ incident duration (min).

- _Inc_ Type _—_ incident severity type (1–5).

- _ninc_ —number of incidents.

- _nj_ —expected frequency of all incidents in the study period for month
_j_, rounded to the nearest integer.

- _n_ Day _,k_ —number of days in the reliability reporting period associated
with demand combination _k._

- _NDC_ —number of demand-level combinations considered.

- _NScen_ —number of scenarios in the analysis.

- _NInc,i_ —number of incidents associated with severity type _i_ .

- _NScen,Inc_ —number of all incident events generated for all scenarios.

- _NScen,j_ —number of scenarios associated with month _j_ of the
reliability reporting period.


for which the work zone is active.

- _P_ { _s_ }— probability of scenario _s_ .

- _P_ t{ _w_, _j_ }—time-wise probability of weather type _w_ in month _j_ .

- _rDC_ —ratio of weekday types with an active work zone in a given
month to the total number of each weekday type occurring in a given
month.

- _VMTi,p_ —vehicle miles traveled on segment _i_ during analysis period
_p_ in the seed file.

- _VMT_ Seed—vehicle miles of travel in the seed file.

- _δx_ —adjustment parameters to satisfy equilibrium calibration
equations.


## **3. UNDERSATURATED SEGMENT EVALUATION**

**FACILITY SPEED CONSTRAINT**

This module begins with the first segment in the first time interval. For
each cell, the flow (or volume) is equal to demand, the volume-to-capacity
ratio is equal to the demand-to-capacity ratio, and undersaturated flow
conditions prevail. Performance measures for the first segment during the
first time interval are calculated by using the procedures for the
corresponding segment type in Chapters 12, 13, and 14.

The analysis continues to the next downstream freeway segment in the
same time interval, and the performance measures are calculated. The
process is continued until the final downstream freeway segment cell in this
time interval has been analyzed. For each cell, the volume-to-capacity ratio
and performance measures are calculated for each freeway segment in the
first time interval. The analysis continues in the second time interval
beginning at the furthest upstream freeway segment and moving downstream
until all freeway segments in that time interval have been analyzed. This
pattern continues for the third time interval, fourth time interval, and so on
until the methodology encounters a time interval that contains one or more
segments with a demand-to-capacity ratio greater than 1.00 or when the final
segment in the final time interval is analyzed. If no oversaturated segments
are encountered, the segment performance measures are taken directly from
Chapters 12, 13, and 14, and the facility performance measures are
calculated as described next in the Directional Facility Module subsection.

When the analysis moves from isolated segments to a facility, an
additional constraint is necessary that controls the relative speed between
two segments. To limit the speeds downstream of a segment experiencing a
low average speed, a maximum achievable speed is imposed on the
downstream segments. This maximum speed is based on acceleration
characteristics reported elsewhere ( _4_ ) and is shown in Equation 25-1.


**Equation 25-1:**


where

_Vmax_ = maximum achievable segment speed (mi/h),

_FFS_ = segment free-flow speed (mi/h),

_Vprev_ = average speed on immediate upstream segment (mi/h), and

_L_ = distance from midpoints of the upstream segment and the subject
segment (ft).


**DIRECTIONAL FACILITY MODULE**

The traffic performance measures can be aggregated over the length of
the directional freeway facility, over the time duration of the study interval,
or over the entire time–space domain. Each measure is discussed in the
following paragraphs.

Aggregating the estimated traffic performance measures over the entire
length of the freeway facility provides facilitywide estimates for each time
interval. Facilitywide travel times, vehicle distance of travel, and vehicle
hours of travel and delay can be computed, and patterns of their variation
over the connected time intervals can be assessed. The computational engine
is limited to 15-min time intervals and 1-min time steps.

Aggregating the estimated traffic performance measures over the time
duration of the study interval provides an assessment of the performance of
each segment along the freeway facility. Average and cumulative
distributions of speed and density for each segment can be determined, and
patterns of the variation over connected freeway segments can be compared.
Average trip times, vehicle distance of travel, and vehicle hours of travel are
easily assessed for each segment and compared.

Aggregating the estimated traffic performance measures over the entire
time–space domain provides an overall assessment over the study interval
time duration. Overall average speeds, average trip times, total vehicle
distance traveled, and total vehicle hours of travel and delay are the most
obvious overall traffic performance measures. Equation 25-2 through


Equation 25-5 show how the facilitywide performance measures are
calculated.

Facility space mean speed in time interval _p_ is calculated with Equation
25-2:


**Equation 25-2:**


Average facility density in time interval _p_ is calculated with Equation 253:


**Equation 25-3:**


Overall space mean speed across all intervals is calculated with
Equation 25-4:


**Equation 25-4:**


Overall average density across all intervals is calculated with Equation
25-5:


**Equation 25-5:**


These performance measures can be compared for different alternatives
to assess the impacts of different volume scenarios or the effects of
geometric improvements to the facility.


## **4. OVERSATURATED SEGMENT EVALUATION**

Oversaturated flow conditions occur when the demand on one or more
freeway segment cells exceeds its capacity. The oversaturated segment
evaluation procedure presented in this chapter is performed separately for
general purpose and managed lanes. To evaluate the effect of interactions
between the general purpose and managed lanes, additional delays are
introduced and calculated in the form of vertical queueing, which is
discussed at the end of this section.

Once oversaturation is encountered, the methodology changes its
temporal and spatial units of analysis. The spatial units become nodes and
segments, and the temporal unit moves from a time interval to smaller time
steps. A node is defined as the junction of two segments. There is always one
more node than there are segments, with a node added at the beginning and
end of each segment. The numbering of nodes and segments begins at the
upstream end and moves to the downstream end, with the segment upstream
of node i numbered segment i – 1 and the downstream segment numbered i,
as shown in Exhibit 25-1. The intermediate segments and node numbers
represent the division of the section between Ramps 1 and 2 into three
segments numbered 2 (ONR), 3 (BASIC), and 4 (OFR). The oversaturated
analysis moves from the first node to each downstream node in the same time
step. After completion of a time step, the same nodal analysis is performed
for subsequent time steps.


**Exhibit 25-1: Node–Segment Representation of a Directional Freeway Facility**


The oversaturated analysis focuses on the computation of segment
average flows and densities in each time interval. These parameters are later
aggregated to produce facilitywide estimates. Two key inputs into the flow
estimation procedures are the time step duration for flow updates and a
flow–density function. These two inputs are described in the next
subsections.


**PROCEDURE PARAMETERS**


**Time Step Duration**

Segment flows are calculated in each time step and are used to calculate
the number of vehicles on each segment at the end of every time step. The
number of vehicles on each segment is used to track queue accumulation and
discharge and to calculate the average segment density.

To provide accurate estimates of flows in oversaturated conditions, the
time intervals are divided into smaller time steps. The conversion from time
intervals to time steps occurs during the first oversaturated time interval and
remains until the end of the analysis. The transition to time steps is essential
because, at certain points in the methodology, future performance estimates
are made on the basis of the past value of a variable.

The computational engine assumes a time step of 15 s for oversaturated
flow computations, which is adequate for most facilities with a minimum
segment length greater than 300 ft. This time step is based on the assumption
that a shockwave of (severe) congestion can travel at speeds up to 20 ft/s or
13.6 mi/h. A minimum segment length of 300 ft ensures that the congestion
shockwave does not travel more than one segment length in one 15-s time
step.

For shorter segments, two problem situations may arise. The first
situation occurs when segments are short and the rate of queue growth
(shockwave speed) is rapid. Under these conditions, a short segment may be
completely undersaturated in one time step and completely queued in another.
The methodology may store more vehicles in this segment during a time step
than space allows. Fortunately, the next time step compensates for this error,
and the procedure continues to track queues and store vehicles accurately
after this correction.


The second situation in which small time steps are important occurs
when two queues interact. There is a temporary inaccuracy due to the
maximum output of a segment changing, thus causing the estimation of
available storage to be slightly in error. This situation results in the storage
of too many vehicles on a particular segment. This “supersaturation” is
temporary and is compensated for in the next time step. Inadequate time step
size will result in erroneous estimation of queue lengths and may affect other
performance measures as well. Regardless, if queues interact, the results
should be viewed with extreme caution.


**Flow–Density Relationship**

Analysis of freeway segments depends on the relationships between
segment speed, flow, and density. Chapter 12, Basic Freeway and Multilane
Highway Segments, defines a relationship between these variables and the
calculation of performance measures in the undersaturated regime. The
freeway facilities methodology presented here uses the same relationships
for undersaturated segments. In other words, when a segment is
undersaturated the computations of this methodology are identical to the
results obtained from Chapters 12, 13, and 14 for basic freeway segments,
weaving segments, and ramp segments, respectively.

The calculations for oversaturated segments assume a simplified linear
flow–density diagram in the congested region. Exhibit 25-2 shows this flow–
density diagram for a segment having a free-flow speed (FFS) of 75 mi/h.
For other FFSs, the corresponding capacities in Chapters 12, 13, and 14
should be used.

The oversaturated regime curve in Exhibit 25-2 is constructed from a
user-specified jam density (default is 190 pc/mi/ln) and the known value of
capacity, defined as the flow at a density of 45 pc/mi/ln. The flow–density
relationship is assumed to be linear between these two points. The slope of
the resulting line describes the speed of the shock wave at which queues
grow and dissipate, as discussed further below. The speed in a congested
segment is obtained from the prevailing density in the segment, read along the
linear flow–density relationship. Details on the theory of kinematic waves in
highway traffic are given elsewhere ( _5_ - _7_ ).


**Exhibit 25-2: Segment Flow–Density Function**


Note: Assumed FFS = 75 mi/h.


**FLOW ESTIMATION**

The oversaturated portion of the methodology is detailed as a flowchart
in Exhibit 25-3. The flowchart is divided into several sections over several
pages. Processes that continue from one section of the flowchart to another
are indicated by capital letters within parallelograms. Computations are
detailed and labeled in the subsections that follow according to each step of
the flowchart.

The procedure first calculates flow variables starting at the first node
during the first time step of oversaturation and followed by each downstream
node and segment in the same time step. After all computations in the first
time step are completed, calculations are performed at each node and
segment during subsequent time steps for all remaining time intervals until
the analysis is completed.


**Exhibit 25-3: Oversaturated Analysis Procedure**


**Segment Initialization: Exhibit 25-3, Steps 1–4**

Steps 1–4 of the oversaturated procedure prepare the flow calculations
for the first time step and specify return points for later time steps. To
calculate the number of vehicles on each segment at the various time steps,
the segments must contain the proper number of vehicles before the queuing
analysis places unserved vehicles on segments. The initialization of each
segment is described below. A simplified queuing analysis is initially
performed to account for the effects of upstream bottlenecks. These
bottlenecks meter traffic downstream of their location. The storage of
unserved vehicles (those unable to enter the bottleneck) on upstream
segments is performed in a later module. To obtain the proper number of
vehicles on each segment, the expected demand ED is calculated. Expected
demand is based on demands for and capacities of the segment and includes
the effects of all upstream segments. The expected demand is the flow of
traffic expected to arrive at each segment if all queues were stacked
vertically (i.e., no upstream effects of queues). In other words, all segments
upstream of a bottleneck have expected demands equal to their actual
demand. The expected demand of the bottleneck segment and all further
downstream segments is calculated by assuming a capacity constraint at the
bottleneck, which meters traffic to downstream segments. The expected
demand _ED_ is calculated for each segment with Equation 25-6:


**Equation 25-6:**


The segment capacity _SC_ applies to the length of the segment. With the
expected demand calculated, the background density _KB_ can be obtained for
each segment by using the appropriate segment density estimation procedures
in Chapters 12, 13, and 14. The background density is used to calculate the
number of vehicles _NV_ on each segment by using Equation 25-7. If there are
unserved vehicles at the end of the preceding time interval, the unserved
vehicles _UV_ are transferred to the current time interval. Here, _S_ refers to the
final time step in the preceding time interval. The (0) term in _NV_ represents
the start of the first time step in time interval _p_ . The corresponding term at the
end of the time step is _NV_ ( _i_, 1, _p_ ).


**Equation 25-7:**


The number of vehicles calculated from the background density is the
minimum number of vehicles that can be on the segment at any time. This
constraint is a powerful check on the methodology because the existence of
queues downstream cannot reduce this minimum. Rather, the segment can
only store additional vehicles. The storage of unserved vehicles is
determined in the segment flow calculation module later in this chapter.


**Mainline Flow Calculations: Exhibit 25-3, Steps 9 and 16–23**

The description of ramp flows follows the description of mainline flows.
Thus, Steps 5–8 and 10–15 are skipped at this time to focus first on mainline
flow computations. Because of skipping steps in the descriptions, some
computations may include variables that have not been described but that
have already been calculated in the flowchart.

Flows analyzed in oversaturated conditions are calculated for every time
step and are expressed in terms of vehicles per time step. The procedure
separately analyzes the flow across a node on the basis of the origin and
destination of the flow across the node. The mainline flow is defined as the
flow passing from upstream segment _i_ - 1 to downstream segment _i_ . It does
not include the on-ramp flow. The flow to an off-ramp is the off-ramp flow.
The flow from an on-ramp is the on-ramp flow. Each of these flows is shown
in Exhibit 25-4 with the origin, destination, and relationship to segment _i_ and
node _i_ .


**Exhibit 25-4: Definitions of Mainline and Segment Flows**


The segment flow is the total output of a segment, as shown in Exhibit 254. Segment flows are calculated by determining the mainline and ramp flows.
The mainline flow is calculated as the minimum of six constraints: mainline
input ( _MI_ ), _MO_ 1, _MO_ 2, _MO_ 3, upstream segment _i_ - 1 capacity, and
downstream segment _i_ capacity, as explained next.


_Mainline Input: Exhibit 25-3, Step 9_

Mainline input _MI_ is the number of vehicles that wish to travel through a
node during the time step. The calculation includes ( _a_ ) the effects of
bottlenecks upstream of the analysis node, ( _b_ ) the metering of traffic during
queue accumulation, and ( _c_ ) the presence of additional traffic during
upstream queue discharge.

_MI_ is calculated by taking the number of vehicles entering the node
upstream of the analysis node, adding on-ramp flows or subtracting off-ramp
flows, and adding the number of unserved vehicles on the upstream segment.
Thus, _MI_ is the maximum number of vehicles that wish to enter a node during
a time step. _MI_ is calculated by using Equation 25-8, where all values have
units of vehicles per time step.


**Equation 25-8:**


_Mainline Output: Exhibit 25-3, Steps 16–21_

The mainline output is the maximum number of vehicles that can exit a
node, constrained by downstream bottlenecks or by merging on-ramp traffic.
Different constraints on the output of a node result in three separate types of
mainline outputs ( _MO_ 1, _MO_ 2, and _MO_ 3).


_Mainline Output 1, Ramp Flows: Exhibit 25-3, Step 16_

_MO_ 1 is the constraint caused by the flow of vehicles from an on-ramp.
The capacity of an on-ramp segment is shared by two competing flows. This
on-ramp flow limits the flow from the mainline through this node. The total
flow that can pass the node is estimated as the minimum of the segment _i_


capacity and the mainline outputs from the preceding time step. The sharing
of Lane 1 (shoulder lane) capacity is determined in the calculation of the onramp. _MO_ 1 is calculated by using Equation 25-9.



**Equation 25-9:**



⎧

⎨



_Mainline Output 2, Segment Storage: Exhibit 25-3, Steps 20 and_
_21_



The second constraint on the output of mainline flow through a node is
caused by the growth of queues on a downstream segment. As a queue grows
on a segment, it may eventually limit the flow into the current segment once
the boundary of the queue reaches the upstream end of the segment. The
boundary of the queue is treated as a shock wave. _MO_ 2 is a limit on the flow
exiting a node due to the presence of a queue on the downstream segment.

The _MO_ 2 limitation is determined first by calculating the maximum
number of vehicles allowed on a segment at a given queue density. The
maximum flow that can enter a queued segment is the number of vehicles that
leave the segment plus the difference between the maximum number of
vehicles allowed on the segment and the number of vehicles already on the
segment. The density of the queue is calculated by using Equation 25-10 for
the linear density–flow relationship shown in Exhibit 25-2 earlier.



**Equation 25-10:**



Once the queue density is computed, _MO_ 2 can be computed by using
Equation 25-11.



**Equation 25-11:**



⎧

⎨


The performance of the downstream node is estimated by taking the
performance during the preceding time step. This estimation remains valid
when there are no interacting queues. When queues interact and the time steps
are small enough, the error in the estimations is corrected in the next time
step.


_Mainline Output 3, Front-Clearing Queues: Exhibit 25-3, Steps_
_17–19_

The final constraint on exiting mainline flows at a node is caused by
downstream queues clearing from their downstream end. These frontclearing queues are typically caused by incidents in which there is a
temporary reduction in capacity. A queue will clear from the front if two
conditions are satisfied. First, the segment capacity (minus the on-ramp
demand if present) for this time interval must be greater than the segment
capacity (minus the ramp demand if present) in the preceding time interval.
The second condition is that the segment capacity minus the ramp demand for
this time interval must be greater than the segment demand for this time
interval. A queue will clear from the front if both conditions in the following
inequality (Equation 25-12) are met.


**Equation 25-12:**


A segment with a front-clearing queue will have the number of vehicles
stored decrease during recovery, while the back of the queue position is
unaffected. Thus, the clearing does not affect the segment throughput until the
recovery wave has reached the upstream end of the front-clearing queue. The
computational engine implementation is simplified by assuming the
downstream segment is fully queued when the _MO3_ constraint is applied. In


the flow–density graph shown in Exhibit 25-5, the wave speed is estimated
by the slope of the dashed line connecting the bottleneck throughput and the
segment capacity points.


**Exhibit 25-5: Flow–Density Function with a Shock Wave**


Note: Assumed FFS = 75 mi/h.


The assumption of a linear flow–density function greatly simplifies the
calculation of the wave speed. The bottleneck throughput value is not
required to estimate the speed of the shock wave that travels along a known
line. All that is required is the slope of the line, which is calculated with
Equation 25-13.


**Equation 25-13:**


The wave speed is used to calculate the wave travel time _WTT_, which is
the time it takes the front queue-clearing shock wave to traverse this segment.
Dividing the wave speed _WS_ by the segment length in miles gives _WTT_ .


The recovery wave travel time is the time required for the conditions at
the downstream end of the current segment to reach the upstream end of the
current segment. To place a limit on the current node, the conditions at the
downstream node are observed at a time in the past. This time is the wave
travel time. This constraint on the current node is _MO_ 3. The calculation of
_MO_ 3 uses Equation 25-14 and Equation 25-15. If the wave travel time is not
an integer number of time steps, then the weighted average performance of
each variable is taken for the time steps nearest the wave travel time. This
method is based on a process described elsewhere (5–7).



**Equation 25-14:**


**Equation 25-15:**



⎧



⎨



_Mainline Flow: Exhibit 25-3, Steps 22 and 23_



The flow across a node is called the mainline flow _MF_ and is the
minimum of the following variables: _MI_, _MO_ 1, _MO_ 2, _MO_ 3, upstream
segment _i_ - 1 capacity, and downstream segment _i_ capacity, as shown in
Equation 25-16.



**Equation 25-16:**



⎧


⎨



⎧


⎨


⎧



⎫



⎨



⎬



In addition to mainline flows, ramp flows must be analyzed. The
presence of mainline queues also affects ramp flows.



**On-Ramp Calculations: Exhibit 25-3, Steps 10–15**



_On-Ramp Input: Exhibit 25-3, Steps 10 and 11_



⎫


⎬


_i_



⎫


⎬


_i_



The maximum on-ramp input _ONRI_ is calculated by adding the on-ramp
demand and the number of vehicles queued on the ramp. The queued vehicles
are treated as unmet ramp demand that was not served in previous time steps.
The on-ramp input is calculated with Equation 25-17.



**Equation 25-17:**



⎧


⎨



⎧


⎨



_On-Ramp Output: Exhibit 25-3, Step 12_



The maximum on-ramp output _ONRO_ is calculated on the basis of the
mainline traffic through the node where the on-ramp is located. The on-ramp
output is the minimum of two values. The first is segment _i_ capacity minus
_MI_, in the absence of downstream queues. Otherwise, the segment capacity is
replaced by the throughput of the queue. This estimation implies that vehicles
entering an on-ramp segment will fill Lanes 2 to _N_ (where _N_ is the number of
lanes on the current segment) to capacity before entering Lane 1. This
assumption is consistent with the estimation of _v_ 12 from Chapter 14, Freeway
Merge and Diverge Segments.


The second case occurs when the Lane 1 flow on segment i is greater
than one-half of the Lane 1 capacity. At this point, the on-ramp maximum
output is set to one-half of Lane 1 capacity. This output limitation implies that
when the demands from the freeway and the on-ramp are very high, there will
be forced one-to-one merging on the freeway from the freeway mainline and
the on-ramp in Lane 1. An important characteristic of traffic behavior is that,
in a forced merging situation, ramp and right-lane freeway vehicles will
generally merge one on one, sharing the capacity of the rightmost freeway
lane (8). In all cases, the on-ramp maximum output is also limited to the
physical ramp road capacity and the ramp-metering rate, if present. The
maximum on-ramp output is an important limitation on the ramp flow.
Queuing occurs when the combined demand from the upstream segment and
the on-ramp exceeds the throughput of the ramp segment. The queue can be
located on the upstream segment, on the ramp, or on both and depends on the
on-ramp maximum output. Equation 25-18 determines the value of the
maximum on-ramp output.



**Equation 25-18:**



⎧


⎨
# ~~⎪~~ ~~⎪~~



⎧


⎨
# ~~⎪~~ ~~⎪~~



⎧



⎧



⎧

⎨



⎨



⎧


⎨
# ~~⎪~~ ~~⎪~~



⎧


⎨
# ~~⎪~~ ~~⎪~~



⎨



⎧

⎨


⎧

⎨
# ~~**⎪**~~ ~~**⎪**~~



⎧

⎨


⎧

⎨
# ~~**⎪**~~ ~~**⎪**~~



⎧

⎨



This model incorporates the maximum mainline output constraints from
downstream queues, not just the segment capacity. This fact is significant


because as a queue spills over an on-ramp segment, the flow through Lane 1
is constrained. This constraint, in turn, limits the flow that can enter Lane 1
from the on-ramp. The values of _MO_ 2 and _MO_ 3 for this time step are not yet
known, so they are estimated from the preceding time step. This estimation is
one rationale for using small time steps. If there is forced merging during the
time step when the queue spills back over the current node, the on-ramp will
discharge more than its share of vehicles (i.e., more than 50% of the Lane 1
flow). This situation will cause the mainline flow past node i to be
underestimated. But during the next time step, the on-ramp flow will be at its
correct flow rate, and a one-to-one sharing of Lane 1 will occur.


_On-Ramp Flows, Queues, and Delays: Exhibit 25-3, Steps 13–15_

Finally, the on-ramp flow is calculated on the basis of the on-ramp input
and output values computed above. If the on-ramp input is less than the onramp output, then the on-ramp demand can be fully served in this time step
and Equation 25-19 is used.


**Equation 25-19:**


Otherwise, the ramp flow is constrained by the maximum on-ramp output,
and Equation 25-20 is used.


**Equation 25-20:**


In the latter case, the number of vehicles in the ramp queue is updated by
using Equation 25-21.


**Equation 25-21:**


The total delay for on-ramp vehicles can be estimated by integrating the
value of on-ramp queues over time. The methodology uses the discrete queue
lengths estimated at the end of each interval _ONRQ_ ( _i_, _S_, _p_ ) to produce
overall ramp delays by time interval.



**Off-Ramp Flow Calculation: Exhibit 25-3, Steps 5–8**



The off-ramp flow is determined by calculating a diverge percentage on
the basis of the segment and off-ramp demands. The diverge percentage
varies only by time interval and remains constant for vehicles that are
associated with a particular time interval. If there is an upstream queue,
traffic may be metered to this off-ramp, which will cause a decrease in the
off-ramp flow. When the vehicles that were metered arrive in the next time
interval, they use the diverge percentage associated with the preceding time
interval. A deficit in flow, caused by traffic from an upstream queue meter,
creates delays for vehicles destined to this off-ramp and other downstream
destinations. The upstream segment flow is used because the procedure
assumes a vehicle destined for an off-ramp is able to exit at the off-ramp
once it enters the off-ramp segment. This deficit is calculated with Equation
25-22.



**Equation 25-22:**



⎧


⎨
# ~~**⎪**~~ ~~**⎪**~~



⎧


⎨
# ~~**⎪**~~ ~~**⎪**~~



⎧



⎨



⎧


⎨



If there is a deficit, then the off-ramp flow is calculated by using the
deficit method. The deficit method is used differently in two specific
situations. If the upstream mainline flow plus the flow from an on-ramp at the
upstream node (if present) is less than the deficit for this time step, then the
off-ramp flow is equal to the mainline and on-ramp flows times the off-ramp
turning percentage in the preceding time interval, as indicated in Equation
25-23.


**Equation 25-23:**


However, if the deficit is less than the upstream mainline flow plus the
on-ramp flow from an on-ramp at the upstream node (if present), then
Equation 25-24 is used. This equation separates the flow into the remaining
deficit flow and the balance of the arriving flow.


**Equation 25-24:**


If there is no deficit, then the off-ramp flow is equal to the sum of the
upstream mainline flow plus the on-ramp flow from an on-ramp at the
upstream node (if present) multiplied by the off-ramp turning percentage for
this time interval according to Equation 25-25.


**Equation 25-25:**


The procedure does not incorporate any delay or queue length
computations for off-ramps.


**Segment Flow Calculation: Exhibit 25-3, Steps 24 and 25**

The segment flow is the number of vehicles that flow out of a segment
during the current time step. These vehicles enter the current segment either
to the mainline or to an off-ramp at the current node. The vehicles that


entered the upstream segment may or may not have become queued within the
segment. The segment flow _SF_ is calculated with Equation 25-26.


**Equation 25-26:**


The number of vehicles on each segment is calculated on the basis of the
number of vehicles that were on the segment in the preceding time step, the
number of vehicles that entered the segment in this time step, and the number
of vehicles that leave the segment in this time step. Because the number of
vehicles that leave a segment must be known, the number of vehicles on the
current segment cannot be determined until the upstream segment is analyzed.
The number of vehicles on each segment _NV_ is calculated with Equation 2527.


**Equation 25-27:**


The number of unserved vehicles stored on a segment is calculated as the
difference between the number of vehicles on the segment and the number of
vehicles that would be on the segment at the background density. The number
of unserved vehicles _UV_ stored on a segment is calculated with Equation 2528.


**Equation 25-28:**


If the number of unserved vehicles is greater than zero, then a queue is
present on the facility upstream of the node in question. The presence of a
queue and congestion indicates that the node capacity is in queue discharge
mode, which means the queue discharge capacity is reduced relative to the
pre-breakdown capacity by a factor _α_ . To account for this queue discharge
effect, Equation 25-29 is applied to any active bottleneck along the facility if


_UV_ ( _i –_ 1, _t_, _p_ ) > 0.001. This tolerance over an absolute value of zero is
necessary to account for potential rounding errors in the procedure.


**Equation 25-29:**


**SEGMENT AND RAMP PERFORMANCE MEASURES**

In the final time step of a time interval, the segment flows are averaged
over the time interval, and the performance measures for each segment are
calculated. If there was no queue on a particular segment during the entire
time interval, then the performance measures are calculated from the
corresponding Chapter 12, 13, or 14 method for that segment. Because there
are _T_ time steps in an hour, the average segment flow rate in vehicles per
hour in time interval _p_ is calculated by using Equation 25-30.


**Equation 25-30:**


If _T_ = 240 (1-h time steps) and _S_ = 60 (1–analysis period time steps),
then _T_ / _S_ = 4. If there was a queue on the current segment in any time step
during the time interval, then the segment performance measures are
calculated in three steps. First, the average number of vehicles _NV_ over a
time interval is calculated for each segment by using Equation 25-31.


**Equation 25-31:**


Second, the average segment density _K_ is calculated by taking the
average number of vehicles _NV_ for all time steps in the time interval and
dividing it by the segment length, as shown by Equation 25-32.


**Equation 25-32:**


Third, the average speed _U_ on the current segment _i_ during the current
time interval _p_ is calculated with Equation 25-33.


**Equation 25-33:**


Additional segment performance measures can be derived from the basic
measures shown in Equation 25-30 through Equation 25-33. Most prominent
is segment delay, which can be computed as the difference in segment travel
time at speed _U_ ( _i_, _p_ ) and at the segment FFS.

The final segment performance measure is the length of the queue at the
end of the time interval (i.e., step _S_ in time interval _p_ ). The length of a queue
_Q_ on the segment, in feet, is calculated with Equation 25-34.


**Equation 25-34:**


**OVERSATURATION ANALYSIS WITHIN MANAGED LANES**

Whenever oversaturated conditions occur (as defined in Chapter 10) on
freeway facilities that contain managed lanes, the freeway facilities
methodology invokes the oversaturated analysis described in this chapter for


both the general purpose and managed lane facilities. The analysis will be
performed separately for each facility, meaning that the queues in either the
general purpose or managed lanes do not interact with each other. For
freeway facilities with managed lanes that do not have any access segments
connecting the two lane groups, performing oversaturated analysis separately
yields accurate performance measures for both the general purpose and
managed lanes.

However, when access segments connect the two lane groups, no method
currently exists to model the queue interaction between the two. In this
situation, the queue spillback between the general purpose and managed
lanes is modeled as a “vertical queue.” The vehicles that cannot enter the
general purpose or managed lane facilities due to the presence of a queue do
not translate into actual queuing on the origin lane group, as shown in Exhibit
25-6.

The freeway facilities methodology keeps track of vehicles that cannot
enter the downstream segment (past the access point) in the form of a vertical
queue, and it releases these vehicles as congestion dissipates. Note that there
are two vertical queues for each access segment, one for vehicles traveling
from the managed to the general purpose lanes, and the other for vehicles
traveling from the general purpose to the managed lanes. Exhibit 25-6 shows
an example of a vertical queue for the first situation. Note that the existence
of a vertical queue does not lead to actual queuing on the managed lane.


**Exhibit 25-6: Vertical Queuing from a Managed Lane Due to Queue Presence on the**
**General Purpose Lanes**


Despite this simplification of queue spillback modeling, the methodology
keeps track of the delays vehicles encounter in the vertical queues. The delay
is computed as the number of vehicles stored in the vertical queue, multiplied
by 15 min of delay in each analysis period. The delay of the vehicles
originating from the managed lanes that are waiting in the vertical queue is
estimated based on Equation 25-35.


**Equation 25-35:**


where

_DML,vert_ = delay incurred by vehicles originating from the managed
lanes waiting in the vertical queue for one 15-min analysis
period (h) and

_NML,vert_ = average number of vehicles originating from the managed
lanes that are waiting in the vertical queue in one analysis
period (veh).


Similar to the vehicle delay in the managed lanes, the delay of vehicles
originating from the general purpose lanes that are waiting in the vertical
queue is estimated based on Equation 25-36.


**Equation 25-36:**


where

_DGP,vert_ = delay incurred by vehicles originating from the general
purpose lanes waiting in the vertical queue for one 15-min
analysis period (h) and

_NGP,vert_ = average number of vehicles originating from the general
purpose lanes that are waiting in the vertical queue in one
analysis period (veh).


## **5. WORK ZONE ANALYSIS DETAILS**

This section provides additional computational details for work zone
analysis on freeway facilities. The analysis of work zones on basic segments
on a facility is described in Chapter 10, Freeway Facilities Core
Methodology; this section provides additional analysis details for work
zones in merge, diverge, and weaving segments, as well as the analysis of
directional crossover work zones. The information provided in this section is
largely based on results from National Cooperative Highway Research
Program Project 03-107 ( _9_ ).


**SPECIAL WORK ZONE CONFIGURATIONS**

The queue discharge rate model predictions explained in Chapter 10
apply to basic freeway segments. These estimates should be adjusted for
special freeway work zone configurations, such as merge segments, diverge
segments, weaving segments, and work zones with directional crossovers.
The relationships presented in this section were derived from fieldcalibrated microsimulation models for the special work zone configurations.

No data were available for the impacts of these work zone configurations
on FFS, and so FFS estimates for these configurations should be used only
when local data are not available. One exception is the FFS for a directional
crossover, which should be estimated from the geometric design of the
configuration, and is used as an input to the queue discharge rate estimation
for that work zone configuration.


**Work Zone Capacity Adjustments for Merge Segments**

The proportion of work zone capacity (in reference to the basic work
zone capacity calculated in Chapter 10) that is allocated to the mainline flow
in a merge segment is presented separately for locations upstream and
downstream of the special work zone activity segment. Exhibit 25-7 shows
an example for a merge area within a construction zone.


**Exhibit 25-7: On-Ramp Merge Diagram for 2-to-1 Freeway Work Zone Configuration**


Note: WZ = work zone.


Exhibit 25-8 through Exhibit 25-12 give the proportion of work zone
capacity allocated to mainline flow in merge, diverge, and directional
crossover segments. For a weaving segment, a predictive model is presented
following those exhibits. In the exhibits, only a subset of potential work zone
configurations is presented, as these are the only ones that were included in
the simulation modeling effort in the original research.

Exhibit 25-8 presents the proportion of available capacity upstream of a
merge area in a construction zone, as a function of work zone lane
configurations, different levels of on-ramp input volumes, and lengths of the
acceleration lane. Upstream of the work zone, the proportion of capacity
available to the mainline movement decreases considerably as the on-ramp
demand increases.


**Exhibit 25-8: Proportion of Work Zone Queue Discharge Rate (Relative to the Basic**
**Work Zone Capacity) Available for Mainline Flow Upstream of Merge Area**










|Work Zone Lane<br>Configuration|On-Ramp Input<br>Demand (pc/h)|Acceleration Lane Length (ft)<br>100 300 500 700 900 1,100 1,300 1,500|
|---|---|---|
|2 to 1|0<br>250<br>500<br>750<br>1,000|1.00 1.00 1.00 1.00 1.00<br>1.00<br>1.00<br>1.00<br>1.00 0.86 0.86 0.86 0.86<br>0.86<br>0.86<br>0.86<br>1.00 0.70 0.70 0.70 0.70<br>0.70<br>0.70<br>0.70<br>1.00 0.53 0.53 0.53 0.53<br>0.53<br>0.53<br>0.53<br>1.00 0.49 0.45 0.40 0.40<br>0.40<br>0.40<br>0.40|
|2 to 2|0<br>250<br>500<br>750<br>1,000|1.00 1.00 1.00 1.00 1.00<br>1.00<br>1.00<br>1.00<br>1.00 0.92 0.92 0.92 0.92<br>0.92<br>0.92<br>0.92<br>1.00 0.84 0.84 0.84 0.84<br>0.84<br>0.84<br>0.84<br>1.00 0.75 0.75 0.75 0.75<br>0.75<br>0.75<br>0.75<br>1.00 0.67 0.67 0.67 0.67<br>0.67<br>0.67<br>0.67|


|3 to 2|0<br>250<br>500<br>750<br>1,000|1.00 1.00 1.00 1.00 1.00 1.00 1.00 1.00<br>1.00 0.95 0.95 0.95 0.95 0.95 0.95 0.95<br>1.00 0.87 0.87 0.87 0.87 0.87 0.86 0.86<br>1.00 0.78 0.78 0.78 0.78 0.78 0.78 0.78<br>1.00 0.70 0.70 0.70 0.70 0.70 0.70 0.70|
|---|---|---|
|4 to 3|0<br>250<br>500<br>750<br>1,000|1.00 1.00 1.00 1.00 1.00<br>1.00<br>1.00<br>1.00<br>1.00 0.97 0.97 0.98 0.98<br>0.98<br>0.98<br>0.98<br>1.00 0.91 0.91 0.91 0.92<br>0.92<br>0.92<br>0.92<br>1.00 0.85 0.85 0.85 0.86<br>0.86<br>0.86<br>0.86<br>1.00 0.79 0.79 0.79 0.79<br>0.80<br>0.80<br>0.80|



The capacity of the merge segment is the same as a basic work zone
segment, with the caveat that the on-ramp flow consumes a portion of the
mainline capacity. As a result, the available capacity upstream of the merge
area leading into the work zone will be reduced once the queue spills back to
the lane drop point. The proportions presented in Exhibit 25-8 approximate
the conditions of a zipper merge configuration, with capacity divided
approximately equally between the on-ramp and the right-most freeway
mainline lane. In other words, the estimates correspond to a worst-case
scenario for mainline flow in terms of available capacity, and a best-case
scenario for the on-ramp movement. Note that the proportions for a 100-ft
acceleration lane length are all 1.0 because on-ramp vehicles will
experience difficulty entering the mainline lanes with the extremely short
acceleration lane. These findings are based on results from microscopic
simulation models of this configuration.

Research (9) shows that the throughput downstream of a merge area is
approximately equal to the upstream queue discharge rate (before the merge)
in most cases, with some configurations actually showing a marginal increase
in flow. This slight increase occurs because additional demand from the onramp is able to more efficiently utilize gaps in the work zone queue discharge
flow without the turbulence effects of the upstream lane drop. This effect was
primarily observed for long acceleration lanes. However, for a more
conservative estimate of work zone operations, it is recommended not to
consider this increase in flow downstream of the merge area regardless of
lane configuration, on-ramp input volume, or acceleration lane length.


**Work Zone Capacity Adjustments for Diverge Segments**


Similar to merge segment analysis, the analysis of diverge segments
distinguishes between the diverge segment portions of the work zone that are
upstream and downstream of the diverge segment. Exhibit 25-9 shows an
example for a diverge area within a construction zone.


**Exhibit 25-9: Off-Ramp Diverge Diagram for a 2-to-1 Freeway Work Zone Configuration**


Note: WZ = work zone.


Exhibit 25-10 presents the proportion of available capacity downstream
of a diverge area for various freeway work zone lane configurations,
different levels of off-ramp volume percentage, and deceleration lane
lengths. Upstream of the diverge area, research ( _9_ ) shows the available
capacity is generally equivalent to that of a basic work zone segment.
Therefore, it is recommended to apply a fixed adjustment of 1.00 upstream of
the diverge area regardless of lane configuration, off-ramp volume
percentage, or deceleration lane length.

At the downstream end, however, the proportion of available capacity for
mainline volume decreases significantly as the off-ramp volume percentage
increases. Analysts should expect work zone operations to improve
downstream of a diverge segment (but still within the work zone) because
some portion of traffic will exit the freeway, thereby decreasing the
processed volume below the downstream capacity. However, if the
deceleration lane lengths are shorter than 100 ft, exiting vehicles will need to
slow down while still on the mainline to complete the exit maneuver. This
speed reduction may drop mainline capacity by as much as 10% or more.

For a diverge area, the proportion of off-ramp demand that can be served
in the work zone under congested conditions can be predicted as presented in


Exhibit 25-11. This proportion is defined as the off-ramp observed volume
divided by the off-ramp demand volume.


**Exhibit 25-10: Proportion of Work Zone Capacity Available for Mainline Flow**
**Downstream of Diverge Area**










|Work Zone Lane<br>Configuration|Off-Ramp<br>Volume<br>Percentage|Deceleration Lane Length (ft)<br>100 300 500 700 900 1,100 1,300 1,500|
|---|---|---|
|2 to 1|0.0<br>6.3<br>12.5<br>18.8<br>25.0|1.00 1.00 1.00 1.00 1.00<br>1.00<br>1.00<br>1.00<br>0.94 0.94 0.94 0.94 0.94<br>0.94<br>0.94<br>0.93<br>0.87 0.88 0.88 0.88 0.88<br>0.88<br>0.87<br>0.87<br>0.79 0.82 0.82 0.82 0.82<br>0.81<br>0.81<br>0.81<br>0.72 0.76 0.76 0.75 0.75<br>0.75<br>0.75<br>0.75|
|2 to 2|0.0<br>6.3<br>12.5<br>18.8<br>25.0|1.00 1.00 1.00 1.00 1.00<br>1.00<br>1.00<br>1.00<br>0.93 0.94 0.94 0.94 0.94<br>0.94<br>0.94<br>0.94<br>0.84 0.87 0.87 0.87 0.87<br>0.87<br>0.87<br>0.87<br>0.76 0.81 0.81 0.81 0.81<br>0.81<br>0.81<br>0.81<br>0.68 0.75 0.75 0.75 0.75<br>0.75<br>0.75<br>0.75|
|3 to 2|0.0<br>6.3<br>12.5<br>18.8<br>25.0|1.00 1.00 1.00 1.00 1.00<br>1.00<br>1.00<br>1.00<br>0.93 0.94 0.94 0.94 0.94<br>0.94<br>0.94<br>0.94<br>0.86 0.87 0.87 0.87 0.87<br>0.87<br>0.87<br>0.87<br>0.78 0.81 0.81 0.81 0.81<br>0.81<br>0.81<br>0.81<br>0.69 0.74 0.74 0.74 0.74<br>0.74<br>0.74<br>0.74|
|4 to 3|0.0<br>6.3<br>12.5<br>18.8<br>25.0|1.00 1.00 1.00 1.00 1.00<br>1.00<br>1.00<br>1.00<br>0.93 0.93 0.93 0.93 0.93<br>0.93<br>0.93<br>0.93<br>0.86 0.87 0.87 0.87 0.87<br>0.87<br>0.87<br>0.87<br>0.76 0.80 0.80 0.80 0.80<br>0.80<br>0.80<br>0.80<br>0.64 0.73 0.73 0.73 0.73<br>0.73<br>0.73<br>0.73|



**Exhibit 25-11: Proportion of Off-Ramp Demand Served in Work Zone**


**Lane Configuration** **Proportion of Off-Ramp Demand Served in Work Zone**

2 to 1 0.39
2 to 2 0.82
3 to 2 0.53
4 to 3 0.60


**Work Zone Capacity Adjustments for Crossover Segments**


Exhibit 25-12 presents the proportion of work zone capacity available
for a directional crossover for various crossover vehicle speeds. As shown
in the exhibit, the crossover capacity is highly sensitive to average crossover
speed. The variation in capacity for different work zone lane configurations
was found to be negligible in crossovers. The estimates in Exhibit 25-12
should be applied as multipliers of the basic segment work zone capacity
described above.


**Exhibit 25-12: Proportion of Available Work Zone Capacity for a Directional Crossover**
**in the Work Zone**







|Lane Configuration|Crossover Average Speed (mi/h)<br>25 35 45|
|---|---|
|2 to 1<br>3 to 2<br>4 to 3|0.83<br>0.90<br>0.94|


**Work Zone Capacity Adjustments for Weaving Segments**

In a weaving area, the proportion of work zone capacity available for
mainline flow can be predicted by using a two-step model. In Step 1, the
analyst estimates the maximum proportion of mainline flow that can be
served through the work zone based on the work zone lane configuration and
the volume ratio. This maximum becomes an upper bound on the actual
estimated proportion, which is estimated in Step 2. In Step 2, the actual
proportion of work zone capacity available for mainline flow is estimated
based on the lane configuration, volume ratio, and auxiliary lane length. The
final proportion of mainline flow that can be processed through the weaving
segment is the lower of the two estimated proportions from Steps 1 and 2.
The model intercept and coefficient values for Equation 25-37 and Equation
25-38 are presented in Exhibit 25-13.


_Step 1: Estimate Maximum Mainline Allocation Proportion_


**Equation 25-37:**


where

_MaxProportion_ = maximum proportion of work zone capacity
available for mainline flow at the weave area
(decimal),

Intercept = model intercept,

_ß1_ = model coefficient for 2-to-1 lane closures,

2-to-1 = indicator variable that is 1 when the work zone has a
2-to-1 configuration and 0 otherwise,

_ß2_ = model coefficient for 2-to-2 lane closures,

2-to-2 = indicator variable that is 1 when the work zone has a
2-to-2 configuration and 0 otherwise,

_ß_ 3 = model coefficient for 3-to-2 lane closures,

3-to-2 = indicator variable that is 1 when the work zone has a
3-to-2 configuration and 0 otherwise,

_ß_ 4 = model coefficient for 4-to-3 lane closures,

4-to-3 = indicator variable that is 1 when the work zone has a
4-to-3 configuration and 0 otherwise,

_ß_ 5 = model coefficient for volume ratio, and

_VR_ = volume ratio = weave volume/total volume.


_Step 2: Predict Mainline Proportion_


**Equation 25-38:**


where

Proportion = proportion of work zone capacity available for mainline
flow (decimal),


_ß_ 6 = model coefficient for auxiliary lane length,

_AuxLength_ = auxiliary lane length (ft), and


all other variables are as defined previously.

The off-ramp demand volume proportion _Prop_ ( _off-ramp_ ) in the weaving
area is estimated from Equation 25-39, with the intercept and model
coefficients given in Exhibit 25-14, and all other variables as defined
previously.


**Equation 25-39:**


**Exhibit 25-13: Model Coefficients for Estimating the Proportion of Work Zone Capacity**
**in a Weaving Segment**


**Model** **Model Term** **Coefficient**



Intercept
_ß_ 1
_ß_ 2
_ß_ 3
_ß_ 4
_ß_ 5

Intercept
_ß_ 1
_ß_ 2
_ß_ 3
_ß_ 4
_ß_ 5

Intercept
_ß_ 1
_ß_ 2
_ß_ 3
_ß_ 4
_ß_ 5
_ß_ 6



1.0023
–0.1197
0.0105
0.0085
0.0000
–0.3048


1.0573
0.1307
–0.0623
0.0494
0.0000
-0.3332


0.8491
–0.0665
0.0061
0.0050
0.0000
–0.4687
9.0956 × 10 [-5]



Step 1: Maximum Proportion


Step 2: Predicted Proportion



Upstream


Downstream


Upstream



Downstream Intercept 0.8962


_ß_ 1
_ß_ 2
_ß_ 3
_ß_ 4
_ß_ 5
_ß_ 6



0.2702
0.0535
0.1073
0.0000
–0.9694
30.5253 × 10 [-5]



**Exhibit 25-14: Model Coefficients for Estimating the Proportion of Off-Ramp Volume**
**Served in the Weaving Area**


**Model** **Model Term** **Coefficient**



Off-Ramp Volume Proportion



Intercept
_ß_ 1
_ß_ 2
_ß_ 3
_ß_ 4
_ß_ 5



0.6162
–0.2201
0.2082
–0.0551
0.0000
0.0850


## **6. PLANNING-LEVEL METHODOLOGY FOR FREEWAY** **FACILITIES**

This section presents a planning-level approach for freeway facility
analysis that is compatible with the operational method presented in Chapter
10, Freeway Facilities Core Methodology. The planning level-approach is
specifically constructed to

1. Use default values for as many of the operational parameters as
practical;

2. Omit the need to enter detailed data about segment attributes (e.g.,
acceleration lane length and detailed weaving section geometry);

3. Aggregate the analysis to a coarser spatial representation, reporting
at the freeway section level instead of the HCM segment level; and

4. Enable HCM users to manually carry out the analysis for a single
peak hour without an extensive computational burden.

The method covers both undersaturated and oversaturated conditions and
produces estimates of travel time, speed, density, and level of service (LOS).
The underlying methodology relies on developing a relationship between
_delay rate_ per unit distance on a basic freeway segment, and the demand-tocapacity ratio. For weaving segments, capacity adjustment factors (CAFs)
are developed based on the volume ratio and segment length. By using these
factors, demand-to-capacity ratios on weaving segments can be adjusted, and
the segment is subsequently treated similarly to a basic freeway segment. The
capacities of merge and diverge segments are determined from the demand
level, FFS, and space mean speed. CAFs are subsequently calculated for
those segments, and their demand-to-capacity ratios are adjusted accordingly.


**INPUT REQUIREMENTS**

Input variables are characterized into global and section inputs. Sections
are defined to occur between points where either demand or capacity


changes, as shown in Exhibit 25-15.


**Exhibit 25-15: Schematics of Freeway Sections**


For instance, the first section in Exhibit 25-15 (starting from the left) is a
basic freeway section. This section is followed by an on-ramp, and the
demand level changes. Capacity and demand remain unchanged until the first
off-ramp. Consequently, the second freeway section in Exhibit 25-15 is
defined as a ramp section. The next section that follows is a basic freeway
section. It is followed by a weaving section (this section is a weaving
section due to the presence of an auxiliary lane). The weaving section is
followed by another ramp section (due to an off-ramp), a basic section, and
finally a ramp section (due to an on-ramp). Introduction of freeway sections
facilitates user input and is more compatible with links in travel demand
models as well as modern digital data sources.

In the operational freeway facilities method, the influence area of an onramp or off-ramp is typically limited to a length of 1,500 ft. In the planning
method, ramp sections can be longer. For cases where a ramp section length
exceeds 2 mi, it is recommended to divide the section into multiple sections
to avoid having the lower ramp section capacity apply for a very long
distance.

Global inputs include information about the facility of interest and are
applicable to all sections across all analysis periods. These inputs include

1. Free-flow speed ( _SFFS_ ),

2. Peak hour factor ( _PHF_ ),

3. Percentage heavy vehicles ( _%HV_ ),


4. General terrain type for truck passenger-car equivalent (PCE)
conversion,

5. _K_ -factor [to convert directional annual average daily traffic (AADT)
to peak hour flows], and

6. Traffic growth factor ( _ftg_ ).

The equation used to estimate section speeds in this planning method
(Equation 25-45) is fully consistent with the basic freeway segment speed–
flow models presented in Chapter 12, Basic Freeway and Multilane
Highway Segments. Section inputs cover information that is applicable to a
given section across all analysis periods and that may vary from one section
to another as a function of

1. Section type (basic, weave, ramp),

2. Section length _L_ (mi),

3. Section number of lanes, and

4. Section directional AADT.

This information, along with the global inputs, is used to calculate the
free-flow travel rate (the inverse of FFS), CAFs for weave and ramp
sections, adjusted lane capacity (the product of base capacity and CAF), and
section capacity (the product of adjusted lane capacity and number of lanes).
The planning methodology follows five basic steps:

1. Demand-level calculations;

2. Section capacity calculations and adjustments;

3. Delay rate estimation;

4. Average travel time, speed, and density calculations; and

5. Level of service.

All steps are described in detail below.


**STEP 1: DEMAND-LEVEL CALCULATIONS**

The demand level for each section is determined from the entering
demand, exiting demand, and carryover demand from a previous analysis


period (in the case of oversaturated conditions).



The methodology uses the directional average annual daily traffic on
section _i AADTi_, _K_ -factor, traffic growth factor _ftg_, and peak hour factor _PHF_
during each 15-min analysis period _t_ in the peak hour to compute the demand
inflow and outflow _Vi,t_ as shown in Equation 25-40:



**Equation 25-40:**



⎧


⎨



where all parameters were defined previously.



All demand inputs should be in units of passenger cars per hour per lane
(pc/h/ln). If demands are given in units of vehicles per hour per lane
(veh/h/ln), they need to be converted with Equation 25-41.



**Equation 25-41:**


where



⎧


⎨
# **⎪**



_qi,t_ = demand flow rate in PCEs (pc/h),



_Vi,t_ = demand flow rate in vehicles per hour (veh/h), and



_fHV_ = adjustment factor for presence of heavy vehicles in traffic stream.



Just as in the operational method, all heavy vehicles are classified as
single-unit trucks (SUTs) or tractor-trailers (TTs). Recreational vehicles and
buses are treated as SUTs. The heavy-vehicle adjustment factor _fHV_ is
computed from the combination of the two heavy vehicle classes, which are
added to get an overall truck percentage _PT_, as shown by Equation 25-42.


**Equation 25-42:**


where

_fHV_ = heavy-vehicle adjustment factor (decimal),

_PT_ = proportion of SUT and TTs in traffic stream (decimal), and

_ET_ = PCE of one heavy vehicle in the traffic stream (PCE).


The values for _ET_ are 2.0 for level terrain and 3.0 for rolling terrain. For
specific grades, Chapter 12 provides other heavy-vehicle equivalency
factors.

The converted demand flow rates _qi,t_ can represent both inflow demand
and outflow demand. For the first facility section and all on-ramps, _qi,t_
represents inflow demand and is denoted by ( _qi,z_ )in. For all off-ramps, _qi,t_
represents outflow demand and is represented by ( _qi,z_ )out.

Demand level _di,p_ (in passenger cars per hour) on section _i_ in analysis
period _p_ is computed as the demand level in section _i_ - 1, plus the inflow at
section _i_ during analysis period _p_, minus the outflow at the same section at
analysis period _p,_ plus any carryover demand _d'i,p_ –1 in section _i_ from the
previous analysis period _p_ - 1 _._ The relationship is as shown in Equation 2543.


**Equation 25-43:**


where all variables are as defined previously.

The carryover demand _d'i,p_ –1 on section _i_ at analysis period _p_ is the
difference between the section demand and capacity, as given by Equation


25-44.


**Equation 25-44:**


The carryover demand is also used as an indication of a queue on the
section. Note that in this approach, queues are stacked vertically and do not
spill back into an upstream link. The section queue length is estimated by
dividing the difference in lane demand and capacity by the density.
Essentially, it provides an estimate for how long the queue would spill back
at the given density, assuming a fixed number of lanes upstream of the
bottleneck.


**STEP 2: SECTION CAPACITY CALCULATIONS AND**
**ADJUSTMENTS**

The capacity of basic freeway sections is found by using the FFS and the
percentage of heavy vehicles on the facility, as shown by Equation 25-45.


**Equation 25-45:**


where _ci_ is the capacity of freeway section _i_ (pc/h/ln) and _FFS_ is the
facility’s free-flow speed (mi/h).

Equation 25-45 provides capacity values for basic freeway sections.
This capacity must be adjusted for weaving, merge, diverge, and ramp
sections, as described next.


**Capacity Adjustments for Weaving Sections**

As mentioned above, the planning method is derived from the basic
freeway segment speed–flow model to estimate a section’s delay rate and
travel speed. When applied to weaving sections, an adjustment to capacity is
required to account for the generally lower capacity in weaving segments.


This capacity adjustment factor _CAF_ weave can be estimated with Equation 2546.


**Equation 25-46:**


where

_CAF_ weave = capacity adjustment factor used for a weaving segment (0 =
_CAF_ weave = 1.0) (decimal),

_Vr_ = ratio of weaving demand flow rate to total demand flow
rate in the weaving segment (decimal), and

_Ls_ = weaving segment length (ft).


Through this capacity adjustment, the basic section method can be
extended to weaving sections, as described elsewhere ( _10_ ). The process for
estimating _CAF_ weave is based on a representative weaving section with the
following characteristics (see Chapter 13 for additional details):

   - Minimum number of lane changes that must be made by a single
weaving vehicle from the on-ramp to the freeway: _LC_ ( _RF_ ) = 1,

   - Minimum number of lane changes that must be made by a single
weaving vehicle from the freeway to the off-ramp: _LC_ ( _FR_ ) = 1,

   - Minimum number of lane changes that must be made by a ramp-toramp vehicle to complete a weaving maneuver: _LC_ ( _RR_ ) = 0, and

   - Number of lanes from which a weaving maneuver may be made with
one or no lane changes: _N_ ( _WL_ ) = 2.


**Adjustments for Ramp Sections**

Research shows an average CAF of 0.9 can be used for ramp sections
with an on-ramp or off-ramp ( _10_, _11_ ). It is recognized that known bottlenecks
may have significantly reduced capacities that require a lower CAF. Further
calibration of the CAF by the analyst is strongly encouraged when applying


this method to on-ramp sections with known capacity constraints and
congestion impacts. Analyst calibration of this factor is also possible for offramp sections.



**STEP 3: DELAY RATE ESTIMATION**



The planning-level approach estimates the delay rate per unit distance as
a function of a section’s demand-to-capacity ratio. The delay rate is the
difference between the actual and free-flow travel time per unit distance. For
example, if a facility’s space mean speed is 60 mi/h relative to an FFS of 75
mi/h for a 0.5-mi segment, then the free-flow travel time is 0.4 min, and the
actual travel time is 0.5 min. The delay rate per mile is the difference of
those travel times divided by the segment length, which gives a delay rate of
0.2 min/mi. The calculation of the delay rate needs to be performed
differently for undersaturated and oversaturated conditions, as described
next.



**Undersaturated Conditions**



For undersaturated conditions, the basic freeway segment speed–flow
model in Chapter 12 can be used to estimate delay rates. However, for a
planning-level analysis, it is desirable to further simplify the estimation of
delay rate to be a function of inputs readily available in a planning context.
The delay rate Δ _RUi,p_ (in minutes per mile) for segment _i_ in analysis period _p_
as a function of the demand-to-capacity ratio _di,p/ci_ is given by Equation 2547.



**Equation 25-47:**



⎧

⎨
# **⎪**



⎧

⎨



where _A_, _B_, _C_, _D_, and _E_ are parameters given in Exhibit 25-16 and all other
variables are as defined previously.


**Exhibit 25-16: Parameter Values for Undersaturated Model**

|Free-Flow Speed (mi/h)|A B C D E|
|---|---|
|75<br>70<br>65<br>60<br>55|68.99<br>–77.97<br>34.04<br>–5.82<br>0.44<br>71.24<br>–85.48<br>35.58<br>–5.44<br>0.52<br>92.45<br>–127.33<br>56.34<br>–8.00<br>0.62<br>121.35<br>–184.84<br>83.21<br>–9.33<br>0.72<br>156.43<br>–248.99<br>99.20<br>–0.12<br>0.82|



**Oversaturated Conditions**

For oversaturated conditions, the additional delay rate is approximated
assuming uniform arrival and departures at the bottleneck location. With the
demand exceeding capacity, any demand that cannot be served through the
bottleneck must be stored upstream of the bottleneck in a queue. The
_additional_ oversaturation delay rate Δ _ROi,p_ (in minutes per mile) for segment
_i_ at analysis period _p_, over a 15-min (900-s) analysis period, is obtained by
Equation 25-48.


**Equation 25-48:**


where all variables are as previously defined.


**STEP 4: AVERAGE TRAVEL TIME, SPEED, AND DENSITY**
**CALCULATIONS**

After the delay rate is determined, the travel rate is computed by
summing the delay rate and travel rate under free-flow conditions, as shown
by Equation 25-49.


**Equation 25-49:**


where _TRi,p_ is the travel rate on segment _i_ in analysis period _p_ (min/mi),
_TRFFS_ is the travel rate under free-flow conditions (min/mi), and all other
parameters are as previously defined.

The section travel time is then computed by multiplying the travel rate
and segment length, as shown by Equation 25-50.


**Equation 25-50:**


where _Ti,p_ is the travel time on segment _i_ in analysis period _p_ (min/mi), _TRi,p_
is the travel rate on segment _i_ in analysis period _p_ (min/mi), and _Li_ is the
length of section _i_ (mi).

The average speed _Si,p_ (in miles per hour) on section _i_ in analysis period
_p_ is computed by using Equation 25-51.


**Equation 25-51:**


Finally, the density is calculated as shown by Equation 25-52.


**Equation 25-52:**


where _Di,p_ is density on section _i_ in analysis period _p_ (pc/mi/ln), _Ni_ is the
number of lanes in section _i_, _di,p_ is section demand (pc/h), and _Si,p_ is speed
(mi/h).

Thus, the planning-level method provides a facility performance
summary that includes whether the facility is undersaturated or oversaturated,


the total facility travel time, the space mean speed, the average facility
density, and the total queue length.


**STEP 5: LEVEL OF SERVICE**

With the density obtained in Step 4, LOS can be estimated for urban or
rural facilities following the thresholds in Chapter 10.

The LOS criteria for urban and rural freeway facilities are repeated in
Exhibit 25-17. Urban LOS thresholds are the same density-based criteria
used for basic freeway segments. Studies on LOS perception by rural
travelers indicate lower-density thresholds than those of their urban freeway
counterparts. The average LOS applies to each 15-min analysis period.


**Exhibit 25-17: LOS Criteria for Urban and Rural Freeway Facilities**


**Urban Freeway Facility Density** **Rural Freeway Facility Density**
**LOS** **(pc/mi/ln)** **(pc/mi/ln)**



A
B
C
D
E
F



≤11
>11–18
>18–26
>26–35
>35–45
>45 or
any component section _vd/c_ ratio > 1.00



≤6
>6–14
>14–22
>22–29
>29–39
>39 or
any component section _vd/c_ ratio >1.00


## **7. MIXED-FLOW MODEL FOR COMPOSITE GRADES**

This section presents the application of the mixed-flow model in the case
of composite grades. The procedure builds on the single-grade methodology
described in Chapter 26, Freeway and Highway Segments: Supplemental,
and uses the same basic set of equations. The procedure computes LOS,
capacity, speed, and density for each segment and for the composite grade as
a whole. Many of the equations in this section are identical to those
presented in Chapter 26, although they have different equation numbers. The
major difference with composite grades is that the analyst must compute the
spot travel rates or spot speeds at the start and end of each segment on the
composite grade as an input to the analysis of the next grade segment.


**OVERVIEW OF THE METHODOLOGY**

The methodology assumes the composite grade both begins and ends with
a long, level segment. The example shown in Exhibit 25-18 has five
segments.


**Exhibit 25-18: Schematic of a Composite Grade**


Exhibit 25-19 presents the methodology flowchart. The remainder of this
section provides the computational details for each step in the process.


**STEP 1: INPUT DATA**


The user must supply the length _dj_ (mi) and the grade _gj_ (decimal) for
each segment _j_, including the tangent segment immediately preceding the
composite grade. In addition, the auto-only free-flow speed _FFS_ (mi/h), peak
hour factor _PHF_ (decimal), the flow rate of mixed traffic _v_ mix (veh/h/ln), and
the fraction of SUTs and TTs in the traffic stream must be specified for the
facility as a whole.


**STEP 2: CAPACITY ASSESSMENT**

Before the composite grade is examined in detail, the capacity of the
individual segments _j_ is determined. A mixed-flow capacity adjustment
factor _CAF_ mix, _j_ converts auto-only capacities into mixed-traffic-stream
capacities. It is computed with Equation 25-53. The third term in this
equation changes for each segment.


**Exhibit 25-19: Mixed-Flow Methodology Overview**


**Equation 25-53:**


where

_CAF_ mix, _j_ = mixed-flow capacity adjustment factor for segment _j_
(decimal),


_CAFao_ = capacity adjustment factor for the auto-only case (e.g., due
to weather or incidents) (decimal),

_CAFT,_ mix = capacity adjustment factor for the percentage of trucks in
mixed-flow conditions (decimal), and

_CAFg,_ mix, _j_ = capacity adjustment factor for grade for segment _j_ in
mixed-flow conditions (decimal).


**CAF for the Auto-Only Case**

Because _CAFao_ is used to convert auto-only capacities into mixed-traffic
capacities, it defaults to a value of 1.0 unless other capacity adjustments are
in effect (e.g., weather, incidents, driver population factor).


**CAF for Truck Percentage**

The CAF for truck percentage _CAFT,_ mix is computed with Equation 25-54.


**Equation 25-54:**


where _PT_ is the total percentage of SUTs and TTs in the traffic stream
(decimal).


**CAF for Grade Effect**

The CAF for grade effect _CAFg,_ mix accounts for the grade severity, grade
length, and truck presence. It is computed by using Equation 25-55 with
Equation 25-56.


**Equation 25-55:**


with


**Equation 25-56:**


where

_ρg,_ mix = coefficient for grade term in the mixed-flow CAF equation
(decimal),

_PT_ = total truck percentage (decimal),

_gj_ = grade of segment _j_ (decimal), and

_dj_ = length of segment _j_ (mi).


Once _CAF_ mix _,j_ is computed, the mixed-flow capacity for each segment _j_ is
calculated with Equation 25-57.


**Equation 25-57:**


where

_C_ mix _,j_ = mixed-flow capacity for segment _j_ (veh/h/ln);

_Cao_ = auto-only capacity for the given FFS, from Exhibit 12-6
(pc/h/ln); and

_CAF_ mix _,j_ = mixed-flow capacity adjustment factor for segment _j_
(decimal).


The procedure identifies the smallest of these capacities and designates it
as _C_ mix. It also notes the segment that produces this capacity as _jc_ . The
capacity _C_ mix is checked against the mixed-flow rate _v_ mix to check if _v_ mix ≥
_C_ mix. If this condition occurs, the system is deemed to be oversaturated, LOS


F is reported, and no further analysis is carried out. However, if _v_ mix < _C_ mix,
the procedure continues.


**STEP 3: SPECIFY INITIAL CONDITIONS**

Starting with Step 3, the methodology analyzes each segment in sequence.
Steps 3 through 6 are repeated for each segment until the final segment on the
composite grade is reached. The main focus is on computing travel times and
speeds for SUTs, TTs, and autos.

Step 3 specifies the initial kinematics-based spot speeds for SUTs and
TTs. The effects of the traffic interaction terms are omitted for the time being.
The focus is on the kinematic behavior of the trucks as they ascend and
descend the individual grades. For the first segment, the initial kinematic spot
speed is the speed for SUTs and TTs on the long, level segment that precedes
the composite grade. For all subsequent segments, it is the kinematic spot
speed at the end of the previous segment. The kinematic spot speeds are
speeds without traffic interaction, which will be added to the final kinematic
spot speeds to obtain final spot speeds of each segment.


**STEP 4: COMPUTE TRUCK SPOT AND SPACE-BASED TRAVEL**
**TIME RATES**

This step computes the SUT and TT space-based travel time rates for
each of the segments and the spot rates at the end of each segment. The
procedure follows a process similar to Step 5 of the mixed-flow model
procedure described in Chapter 26.

The first substep involves analyzing the kinematic behavior of the trucks
on the grade. The final spot rates are needed, as well as a determination of
whether the trucks accelerated or decelerated on the grade.

Exhibit 25-20 and Exhibit 25-21 can be used for these purposes. These
graphs are based on kinematic relationships given elsewhere ( _12_ ).
Alternative models of propulsive and resistive forces, such as more complex
ones that account for gear shifting (e.g., _13_, _14_ ), can produce longer travel
times. Such considerations can be incorporated into the mixed-flow model by
adjusting the parameter values that affect the tractive effort to account for the
additional losses. The travel time rates presented here are based on a model


that assumes constant peak-engine power. Other models (e.g., _13_, _14_ ) account
for the power losses that occur for the time intervals prior to and after gear
shifting when the engine speed is outside the range that produces peak power.

Exhibit 25-20 shows the trends in SUT spot rates for various grades
starting from travel rates of 48 s/mi (75 mi/h) and 120 s/mi (30 mi/h).
Exhibit 25-21 shows the same trends for a TT. Clearly, trucks decelerate as
upgrades become steeper. For milder grades, trucks can often accelerate.


**Exhibit 25-20: SUT Spot Rates Versus Distance with Initial Speeds of 75 and 30 mi/h**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100. Solid curves are
for an initial speed of 75 mi/h (48 s/mi) and dashed curves are for an initial speed of
30 mi/h (120 s/mi).


**Exhibit 25-21: TT Spot Rates Versus Distance with Initial Speeds of 75 and 20 mi/h**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100. Solid curves are
for an initial speed of 75 mi/h (48 s/mi) and dashed curves are for an initial speed of
20 mi/h (180 s/mi).


In both Exhibit 25-20 and Exhibit 25-21, the _x_ -axis gives the distance _d_
traveled by the truck, and the _y_ -axis gives the spot travel rate _τkin,j_ at the end
of that distance. The different curves are for various upgrades and
downgrades.

To ascertain whether trucks accelerate or decelerate on segment _j_,
consider the travel time rate trends shown in Exhibit 25-20 and Exhibit 2521. If an SUT’s final spot rate for segment _j τSUT,kin,f,j_ is greater than the
SUT’s initial spot rate for segment _j τSUT,kin,i,j_ and the TT’s spot rate at the
end of segment _j τTT,kin,f,j_ is greater than the TT’s spot rate at the beginning of
segment _j τTT,kin,i,j_, then both truck classes decelerate. If _τSUT,kin,f,j_ <
_τSUT,kin,i,j_ and _τTT,kin,f,j_ < _τTT,kin,i,j_, then both truck classes accelerate.


To determine the end-of-grade spot travel time rates, start by finding the
point on the applicable grade that corresponds to the initial kinematic rate.
Treat that point as the zero distance location. Next, proceed along the grade
length ( _x_ axis) for a distance equal to the length _d_ of the segment and read the
spot rate at that distance. This reading is the final spot rate. For example, an
SUT travels 2,000 ft starting from 60 mi/h (60 s/mi) on a 5% grade. Point 1
in Exhibit 25-20 is the 60-mi/h speed (60-s/mi rate) from which the SUT
starts to travel on the 5% grade. Point 2 is the distance that is treated as the
zero distance of the SUT. Point 3 represents the distance the SUT has
traveled after 2,000 ft. The final spot rate can be read at Point 4. The initial
kinematic SUT and TT spot rates for segment _j τSUT,kin,i,j_ and _τTT,kin,i,j_ are the
kinematic spot rates at the end of the preceding segment. For remaining
segments, _τSUT,kin,i,j_ and _τTT,kin,i,j_ are the kinematic spot rates at the end of the
preceding segment _j_ - 1, which are _τSUT,kin,f,j–1_ and _τTT,kin,f,j–1_ .

The second substep involves determining the space-based travel time
rates for SUTs and TTs. Exhibit 25-22 and Exhibit 25-23 provide examples.
Exhibit 25-22 shows the time versus distance relationships for SUTs starting
at 70 mi/h with a desired speed of 75 mi/h as they accelerate or decelerate
on various grades. Exhibit 25-23 shows time versus distance relationships
for SUTs starting at 30 mi/h as they ascend or descend grades. Relationships
for a range of initial rates for both SUTs and TTs are provided in Appendix
A.

In all exhibits, the _x_ -axis is the distance _d_ traveled by the truck, while the
_y_ axis is the travel time _T_ to cover the grade length _d_ . The various curves in
each exhibit represent different upgrades. All the truck profiles have a
desired speed of 75 mi/h. For example, the 2% curve in Exhibit 25-23 shows
travel time versus distance for SUTs starting from 30 mi/h with a desired
speed of 75 mi/h.

When necessary, symbols are placed on the curves to indicate where a
truck reaches 55, 60, 65, and 70 mi/h, for use when the speed limit is less
than 75 mi/h, as indicated in the notes for Exhibit 25-23. For example, if the
speed limit is 55 mi/h, it is assumed trucks will maintain a constant speed of
55 mi/h after reaching that speed. The analyst would use the graph to
determine the travel time to accelerate to 55 mi/h and then perform the
remainder of the travel time calculation using 55 mi/h as the truck speed. Not


all curves have these symbols, as (a) the truck’s crawl speed would be less
than 55 mi/h for the particular grade, (b) the truck would take more than
10,000 ft to reach that speed, or (c) the graph being used starts from a
relatively high speed (e.g., Exhibit 25-22).


**Exhibit 25-22: SUT Travel Time Versus Distance Curves for 70-mi/h Initial Speed**


Note: Curves in this graph assume a weight-to-horsepower ratio of 100.


**Exhibit 25-23: SUT Travel Time Versus Distance Curves for 30-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.
The analyst should use the Appendix A graph that has a starting spot
speed closest to the value computed in the first substep. Because the graphs
are provided in 5-mi/h increments, this choice means using the graph that is
within 2.5 mi/h of the speed corresponding to the segment’s initial spot rate.

The kinematic space-based travel time rate _τkin_ (in seconds per mile) can
then be computed with Equation 25-58.


**Equation 25-58:**


where _T_ is the segment travel time (s) and _d_ is the grade length (mi).

The maximum grade length shown in the graphs is 10,000 ft. When the
grade length exceeds 10,000 ft, the travel rate can be computed using
Equation 25-59.


**Equation 25-59:**


where

_τkin_ = kinematic travel rate (s/mi),

_T_ 10000 = travel time at 10,000 ft (s),

_δ_ = slope of the travel time versus distance curve (s/ft),

_d_ = grade length (mi), and

5,280 = number of feet in 1 mi.


The _δ_ values for SUTs and TTs are shown in Exhibit 25-24 and Exhibit
25-25, respectively.


**Exhibit 25-24:** _**δ**_ **Values for SUTs**







|Grade|Free-Flow Speed (mi/h)<br>50 55 60 65 70 75|
|---|---|
|–5%<br>0%<br>2%<br>3%<br>4%<br>5%<br>6%<br>7%<br>8%|0.0136<br>0.0124<br>0.0114<br>0.0105<br>0.0097<br>0.0091<br>0.0136<br>0.0124<br>0.0114<br>0.0105<br>0.0097<br>0.0091<br>0.0136<br>0.0124<br>0.0114<br>0.0105<br>0.0100<br>0.0099<br>0.0136<br>0.0124<br>0.0114<br>0.0113<br>0.0112<br>0.0112<br>0.0136<br>0.0129<br>0.0128<br>0.0128<br>0.0128<br>0.0127<br>0.0146<br>0.0146<br>0.0146<br>0.0146<br>0.0145<br>0.0145<br>0.0165<br>0.0165<br>0.0165<br>0.0165<br>0.0165<br>0.0165<br>0.0186<br>0.0186<br>0.0186<br>0.0186<br>0.0186<br>0.0186<br>0.0208<br>0.0208<br>0.0208<br>0.0208<br>0.0208<br>0.0208|


**Exhibit 25-25:** _**δ**_ **Values for TTs**








|Grade|Free-Flow Speed (mi/h)<br>50 55 60 65 70 75|
|---|---|
|–5%<br>0%<br>2%<br>3%|0.0136<br>0.0124<br>0.0114<br>0.0105<br>0.0097<br>0.0091<br>0.0136<br>0.0124<br>0.0114<br>0.0105<br>0.0097<br>0.0091<br>0.0136<br>0.0124<br>0.0119<br>0.0118<br>0.0116<br>0.0115<br>0.0143<br>0.0143<br>0.0142<br>0.0141<br>0.0140<br>0.0138|


Once the end-of-grade spot travel time rates and the space-based rates
are obtained for the current segment, Equation 25-60 and Equation 25-61 are
used to account for the traffic interaction term to obtain the actual truck spot
and space-based travel time rates.


**Equation 25-60:**


**Equation 25-61:**


where

    - = placeholder that can either be _f_ to designate the spot travel
time rate at the end of the segment or _S_ to indicate the
space-based rate across the segment,

_τ*,SUT,j_ = spot travel time rate for SUTs at the end of segment _j_ or the
space-based rate (s/mi),

_τ*,SUT,kin,j_ = kinematic final spot travel time rate or space-based rate
for SUTs (s/mi),

_ΔτTI_ = traffic interaction term (s/mi) from Equation 25-62,

_τ*,TT,j_ = spot travel time rate for TTs at the end of segment _j_ or the
space-based rate (s/mi), and

_τ*,TT,kin,j_ = kinematic final spot travel time rate or space-based rate
for TTs (s/mi).


The traffic interaction term represents the contribution of other traffic to
truck speeds or travel time rates in mixed flow. It is computed by Equation
25-62.



**Equation 25-62:**



where



_ΔτTI_ = traffic interaction term (s/mi),



_Sao_ = auto-only speed for the given flow rate (mi/h) from Equation
25-63,



_FFS_ = base free-flow speed of the basic freeway segment (mi/h),
and



_CAF_ mix = mixed-flow capacity adjustment factor for the segment
(decimal) from Equation 25-53.



The auto-only travel time rate for the given flow rate can be computed
with Equation 25-63.



⎫

⎬
# **⎪**



**Equation 25-63:**



⎧

⎨



⎫

⎬



where



⎧

⎨



_Sao_ = auto-only speed for the given flow rate (mi/h),



_FFS_ = base free-flow speed of the basic freeway segment (mi/h),



_Cao_ = base segment capacity (pc/h/ln) from Exhibit 12-6,


_BPao_ = breakpoint in the auto-only flow condition (pc/h/ln) from
Exhibit 12-6,



_Dc_ = density at capacity = 45 pc/mi/ln,



_v_ mix = flow rate of mixed traffic (veh/h/ln), and



⎤


⎦


⎤


⎦
# ⎥ ⎥



⎤


⎦


⎤


⎦
# ⎥ ⎥



_CAF_ mix = mixed-flow capacity adjustment factor for the basic freeway
segment (decimal).



**STEP 5: COMPUTE AUTOMOBILE SPOT AND SPACE-BASED**
**TRAVEL TIME RATES**



Whether trucks accelerate or decelerate, the automobile spot travel time
rates at the end of the segment are computed with Equation 25-64. The
analyst should check that the automobile spot rates are always less than or
equal to the truck spot rates (i.e., automobile speeds are greater than or equal
to truck speeds).



**Equation 25-64:**


where



⎡


⎣
# ⎢



⎡


⎣
# ⎢



⎡



⎤



⎣



⎦



⎡



⎤



⎣



⎦



_τf,a,j_ = end-of-grade spot travel time rate for automobiles (s/mi),


_τf,SUT,kin,j_ = spot kinematic travel time rate of SUTs at the end of
segment _j_ (s/mi),



_τf,TT,kin,j_ = spot kinematic travel time rate of TTs at the end of segment
_j_ (s/mi),



_ΔτTI_ = traffic interaction term (s/mi),



_v_ mix = flow rate of mixed traffic (veh/h/ln),



⎤


⎦


⎤


⎦
# ⎥



⎤


⎦


⎤


⎦
# ⎥



_FFS_ = base free-flow speed of the basic freeway segment (mi/h),



_PSUT_ = proportion of SUTs in the traffic stream (decimal), and



_PTT_ = proportion of TTs in the traffic stream (decimal).



In Step 4, it was determined whether trucks accelerate or decelerate
across a segment. If they decelerate, Equation 25-65 is used to compute the
auto space-based travel time rate. If trucks accelerate, Equation 25-66 is
employed. The auto space mean rates are always less than or equal to the
truck space mean rates.



**Equation 25-65:**


**Equation 25-66:**



⎡


⎣
# ⎢



⎡


⎣
# ⎢



⎡



⎤



⎣



⎦



⎡



⎤



⎣



⎦


⎡



⎤



⎣



⎦



⎡



⎤



⎣



⎦



where



⎡


⎣
# ⎢



⎡


⎣
# ⎢



_τS,a,,j_ = auto space-based travel time rate (s/mi),



⎤


⎦


⎤


⎦
# ⎥



⎤


⎦


⎤


⎦
# ⎥



_τS,SUT,kin,j_ = kinematic space-based travel time rate of SUTs (s/mi),



_τS,TT,kin,j_ = kinematic space-based travel time rate of TTs (s/mi),



_ΔτTI_ = traffic interaction term (s/mi),



_v_ mix = flow rate of mixed traffic (veh/h/ln),



_FFS_ = base free-flow speed of the basic freeway segment (mi/h),



_PSUT_ = proportion of SUTs in the traffic stream (decimal), and



_PTT_ = proportion of TTs in the traffic stream (decimal).



The traffic interaction term is the same for all the travel time rate
equations and can be computed with Equation 25-62.



**STEP 6: COMPUTE MIXED-FLOW SPACE-BASED TRAVEL TIME**
**RATE AND SPEED**


The mixed-flow space-based travel time rate _τ_ mix _,j_ and the space-based
speed _S_ mix _,j_ are computed with Equation 25-67 and Equation 25-68,
respectively.


**Equation 25-67:**


**Equation 25-68:**


where

_τ_ mix _,j_ = mixed-flow space-based travel time rate for segment _j_ (s/mi),

_τS,a,j_ = automobile space-based travel time rate for segment _j_ (s/mi),

_τS,SUT,j_ = space-based travel time rate of SUTs (s/mi),

_τS,TT,j_ = space-based travel time rate of TTs (s/mi),

_PSUT_ = proportion of SUTs in the traffic stream (decimal), and

_PTT_ = proportion of TTs in the traffic stream (decimal).


As indicated above, Steps 3 through 6 are repeated for each segment until
the end of the composite grade is reached.


**STEP 7: OVERALL RESULTS**

Once spot and space mean speeds and travel time rates have been
developed for all vehicle types on all segments, the overall performance of
the composite grade can now be estimated. The mixed-flow travel time for
each segment can be computed with Equation 25-69.


**Equation 25-69:**


where

_t_ mix _,j_ = mixed-flow travel time segment _j_ (s),

_dj_ = grade length of segment _j_ (mi), and

_S_ mix _,j_ = mixed-flow speed for segment _j_ (mi/h).


The overall mixed-flow travel time _t_ mix _,oa_ is the summation of mixedflow travel times on all segments. The overall space-based travel speed can
then be computed with Equation 25-70.


**Equation 25-70:**


where

_S_ mix _,oa_ = overall mixed-flow speed (mi/h);

_doa_ = overall distance, the summation of all the segment grade
lengths on the composite grade (mi); and

_t_ mix _,oa_ = overall mixed-flow travel time (s).


## **8. FREEWAY CALIBRATION METHODOLOGY**

This section presents a calibration methodology for the procedures
described in Chapter 10, Freeway Facilities Core Methodology, and Chapter
11, Freeway Reliability Analysis. The freeway calibration methodology is
carried out at three main levels:

1. Calibration at the core freeway facility level,

2. Calibration at the reliability level, and

3. Calibration at the Active Traffic and Demand Management (ATDM)
strategy assessment level.

The procedure uses _sequential calibration_ to calibrate these three
distinct methodological parts, meaning that the calibration is carried out
sequentially for each level. After a level is fully calibrated, no further change
is allowed from a different level. As a result, this approach requires that the
calibration parameters of different levels be mutually exclusive.

The approach first calibrates the base scenario, then focuses on
reliability-level calibration, and concludes with ATDM-level calibration. It
is logical both that the base scenario (i.e., core freeway facility) should be
fully calibrated before evaluating reliability or ATDM strategies and that the
base scenario calibration should not be affected by any subsequent changes
from the reliability or ATDM calibration levels. Consequently, it is critical
to select a suitable base scenario with oversaturated flow conditions to
ensure that the bottlenecks are calibrated appropriately. More information
about the development of the methodology is provided in a paper ( _15_ )
located in the Technical Reference Library section of online HCM Volume 4.

Calibration relies on field measurements of key input variables, including
the segment capacity. Chapter 12, Basic Freeway and Multilane Highway
Segments, provides definitions for prebreakdown and queue discharge
capacity. Chapter 26, Freeway and Highway Segments: Supplemental,


provides guidance for field measuring and estimating capacity from sensor
data.


**CALIBRATION AT THE CORE FREEWAY FACILITY LEVEL**

The core freeway facility analysis is calibrated for a specific day, called
the _seed day._ Exhibit 25-26 depicts five steps of the calibration process for a
core facility analysis. After gathering input data, the actual calibration
consists of three steps (Steps 2, 3, and 4), the order of which is somewhat
flexible. Multiple iterations may be needed to achieve satisfactory
performance. A detailed explanation of each step follows.


**Step 1: Gather Input Data**

In this step, all input data required for a single freeway facility analysis
(computational engine seed file) need to be gathered. These data include

1. Geometric information such as segment type, segment length, and
number of lanes;

2. Facility free-flow speed (FFS);

3. Capacity estimate for bottleneck segment(s); and

4. Demand-level data for all segments in all time intervals.

Geometric data are model input parameters and will not be changed in
the calibration process. The other three inputs (FFS, capacity, and demand)
are used as calibration parameters.


**Exhibit 25-26: Calibration Steps for the Core Freeway Facility Level**


**Step 2: Calibrate Free-Flow Speed**

FFS can be field measured or estimated by using the procedure given in
Chapter 12. The FFS calibration procedure may be applied in either case;
however, if accurate field measurements of FFS are available, great care
should be taken before changing a field-measured input.

To start, the analyst should select a time interval with a low demand
level and no active bottleneck. The analyst should then compare the
estimated free-flow travel time of this interval with the field measurements.
Because a later step requires the analyst to look at congested periods, the


study period should be sufficiently long to include free-flow conditions
before or after the onset of congestion.

The calibration process involves making a computational engine run for
the seed day, recording the average travel time for a low-demand time
interval, and comparing it to the observed travel time. The user needs to
repeatedly perform one of the following actions until the predicted facility
travel time is within a predefined threshold (e.g., 10% error tolerance) of the
observed facility travel time:

   - _Reduce_ the FFS in 1- to 5-mi/h increments if the predicted travel time
is _less_ than the observed travel time, or

   - _Increase_ the FFS in 1- to 5-mi/h increments if the predicted travel
time is _more_ than the observed travel time.

This process should only be used for analysis periods with demand
levels far less than oversaturation (i.e., free-flow conditions). The speed–
flow diagram in Exhibit 25-27 illustrates the effect of different FFSs on the
overall facility speed–flow–density relationship. A higher free-flow speed
_FFS_ 1 and a lower free-flow speed _FFS_ 2 are shown. A 5-mi/h drop in FFS is
associated with a drop in capacity equal to 50 pc/h/ln, except at very high
FFSs.


**Exhibit 25-27: Effect of Calibrating Free-Flow Speed on Capacity**


**Step 3: Calibrate Bottleneck Capacity**

In this step, the location and extent of bottlenecks are calibrated, which
requires a freeway facility to feature at least some periods of oversaturated
flow conditions. Guidance for selecting capacity measurement locations and
for reducing the collected data is provided in Chapter 26.

It is very important to calibrate for capacity, as research ( _11_ ) shows the
controlling capacity at the bottleneck is often significantly less than the
HCM’s base capacity. Three parameters are used to calibrate for the location
and extent of bottlenecks:

1. _Prebreakdown capacity_ at the bottleneck, implemented through a
capacity adjustment factor (CAF) relative to the base capacity for a
freeway segment. In the HCM, the prebreakdown flow rate is defined
as the 15-min average flow rate immediately prior to the breakdown
event. For the purposes of this chapter, the prebreakdown flow rate is
equivalent to the segment capacity;


2. _Queue discharge rate_ at the bottleneck following breakdown, as
implemented through a percentage capacity drop α _._ In the HCM, the
queue discharge rate is defined as the average flow rate during
oversaturated conditions (i.e., during the time interval after
breakdown and prior to recovery); and

3. _Jam density_ of the queue forming upstream of the bottleneck, which
describes the maximum density (minimum intervehicle spacing) in a
queued condition.

The prebreakdown capacity and the queue-discharge capacity loss
influence the actual throughput of the bottleneck, as well as the speed of
shock waves describing the rate of change of the back of the queue. Jam
density does not affect throughput; it only influences the formation and
dissipation of queues at a bottleneck. The following exhibits illustrate the
effects of these three calibration parameters in a shock wave diagram format.

In Exhibit 25-28, the number 1 denotes the base condition (dashed gray
line) and the number 2 denotes the alternative condition (solid gray line).
Two demand levels _D_ are shown. Demand rates that are greater than the
bottleneck capacity are noted with an asterisk.


**Exhibit 25-28: Effects of Segment Capacity**


Reducing the prebreakdown capacity increases the speed of the forming
shock wave, but the speed of the recovery wave is decreased. As a result, a


reduction in the segment’s prebreakdown capacity is expected to increase
congestion throughout the segment. Note that it is assumed a reduction in the
segment capacity has no impact on the queue discharge rate at the bottleneck
in the example above. The effects of a drop in queue discharge rate are
shown in Exhibit 25-29.


**Exhibit 25-29: Effects of Queue Discharge Rate Drop**


Exhibit 25-29 shows that including a queue discharge rate drop in the
freeway model results in a reduction in bottleneck throughput after
breakdown. The factor _α_ describes the percentage reduction from
prebreakdown capacity to queue discharge rate. A larger _α_ corresponds to a
larger drop and lower throughput. Implementing this factor results in a drop
in throughput, an increase in the speed of the forming shockwave, and a
decrease in the speed of the recovery wave. The result is a threefold effect
that leads to a higher level of congestion, which has been demonstrated in the
literature ( _16_ ). It is therefore expected that the capacity drop has a nonlinear
effect on the overall facility performance.

Exhibit 25-30 shows the effect of an increase in the jam density on wave
speeds. Interestingly, an increase in the jam density value reduces both the
forming and recovery wave speeds, thus canceling each other’s effects to
some degree. The opposite situation occurs if jam density is decreased, in
which case both the forming and recovery speeds will increase. Although
jam density is likely to affect the queue size (a higher jam density results in a


smaller queue size), it may not influence travel time values as much as the
prebreakdown capacity and queue discharge rate do.


**Exhibit 25-30: Effects of Jam Density**


To calibrate for bottlenecks, the analyst needs to change the capacity and
capacity drop values for different segments of the freeway facility to recreate
the bottlenecks that are observed in the field. Therefore, the analyst must first
identify recurring bottlenecks in the field.

Next, the calibration process begins with setting the segment capacity to
the HCM value for the facility’s FFS (e.g., 2,400 pc/h/ln for a 70-mi/h FFS).
A value of 7% for capacity drop is recommended.

If these initial values predict the bottleneck location correctly, the
analysis proceeds to the validation step. If the model fails to identify a
bottleneck, the analyst should reduce capacity in increments of 50 pc/h/ln
until a bottleneck occurs. However, if the HCM model identifies a bottleneck
that does not exist in the field, the analyst should increase capacity in
increments of 50 pc/h/ln until the bottleneck disappears.

It is recommended that analysts wait to adjust the capacity drop value
until after the bottleneck locations have been fixed. This procedure is
performed as part of validating the queue length and travel time, as explained
in Step 5.


**Step 4: Calibrate Facility Demand Level**

The demand level is a model input that can serve as a calibration
parameter as a last resort. Presumably, demand has been measured based on
field data, and therefore can be considered to be a fixed input. However,
given the variability of demand (i.e., day-to-day fluctuation), as well as
potential errors in volume and demand measurements, demand can become a
calibration parameter after the FFS and capacity adjustment possibilities
have been exhausted.

Two potential problems may be encountered with demand levels. First,
in oversaturated conditions, it is not possible to measure the demand level
downstream of a bottleneck or within a queued segment. The volume served
is measured, rather than the demand level. Second, demand data vary from
day to day, and the selected demand levels may not represent a “typical” day.
This second problem is also true if AADT demand values are used to
estimate peak period demands. As a result, although demand level is one of
the inputs to the core freeway facility analysis, it may be subject to
calibration.

To provide an example of the effect of the demand level on segment and
facility travel time, a shockwave representation of the oversaturation model
used in the core HCM freeway facilities methodology is presented. Although
the HCM uses an adaptation of the cell-transmission model to estimate queue
propagation and dissipation patterns at a bottleneck, the shockwave approach
is useful to illustrate the calibration concepts here.

Exhibit 25-31 shows the flow–density relationship under high- and lowvolume conditions for a segment that is just upstream of a bottleneck with a
reduced capacity. As before, the number 1 denotes the base condition
(dashed gray line), the number 2 denotes the alternative condition (solid gray
line), and demand rates greater than the bottleneck capacity are denoted with
an asterisk.


**Exhibit 25-31: Effect of Demand Level**


In Exhibit 25-31 it is evident that an overall increase in demand level
(from _D_ 1* to _D_ 2* and from _D_ 1 to _D_ 2) would result in both an increase in the
forming shock wave speed and a reduction in the recovery wave speed,
assuming a fixed bottleneck capacity. In other words, an overall increase in
demand level results in a higher level of congestion throughout. The greater
the difference between upstream demand and downstream bottleneck
capacity, the faster the resulting shock wave either grows the queue (demandto-capacity ratio > 1.0) or dissipates the queue (demand-to-capacity ratio =
1.0).

The analyst should increase the demand level in increments of 50 pc/h/ln
until all bottlenecks that are observed in the field are activated in the freeway
facility core analysis. However, if the model predicts bottlenecks that do not
exist in the field, the user should decrease the demand level in increments of
50 pc/h/ln until those bottlenecks are deactivated. This activity should be
performed in conjunction with Step 3: Calibrate Bottleneck Capacity.


**Step 5: Validate Travel Time and Queue Length**

The validation step has two major components:

1. Validate facility travel time, and


2. Validate queue length at active bottlenecks.


_Travel Time Validation_

After fixing the FFS and the bottleneck locations, the analyst should
adjust the calibration parameters further to match predicted and observed
facility travel times within a defined range (a 10% or less difference is
recommended). Note that FFS has already been fixed in Step 3 and will not
be adjusted further in this step. This process can be done by adjusting

1. Demand level,

2. Prebreakdown capacity,

3. Capacity drop, and

4. Jam density.

The analyst is trying to match reasonably well the estimated and
observed facility and segment travel times. If the model _underestimates_ the
travel time, the analyst should consider one of the following actions:

1. Increase the demand level (in increments of 100 pc/h/ln),

2. Reduce prebreakdown capacity (in increments of 100 pc/h/ln), or

3. Increase the capacity drop (in increments of 1%).

If the model _overestimates_ travel time, the analyst should consider one of
the following actions:

1. Reduce the demand level (in increments of 50 pc/h/ln),

2. Increase prebreakdown capacity (in increments of 50 pc/h/ln), or

3. Reduce the capacity drop (in increments of 1%).

Note that jam density is unlikely to have a significant impact on facility
travel time and is therefore not included in the steps above.


_Queue Length Validation_

After the facility travel time is fixed, the queue lengths at the facility’s
active bottlenecks should be matched reasonably well (i.e., within 10%)
through further adjustments to the capacity drop and jam density.


If the predicted queue length at an active bottleneck is _shorter_ than
observed in the field, the capacity drop should be _increased_ and the jam
density should be _decreased_ .

However, if the predicted queue length is _longer_ than that observed in the
field, the capacity drop should be _decreased_ and the jam density should be
_increased_ . It is recommended that the capacity drop be changed in increments
of 1% and that the jam density be changed in increments of 10 pc/mi/ln.


**CALIBRATION AT THE TRAVEL TIME RELIABILITY LEVEL**

After calibrating the core freeway facility methodology and fixing the
value of its parameters, a comprehensive travel time reliability calibration is
performed. Note that the process does not allow any change in the parameters
that were calibrated in the previous step. The process requires a host of
different input variables and calibration parameters. Comprehensive
reliability-level calibration, as shown in Exhibit 25-32, starts with gathering
the necessary input data. Some of these parameters, including facility
geometry and FFS, are already known and fixed.

The process includes three major steps: whole-year demand calibration,
incident calibration, and weather calibration. In the rest of this section, each
step is presented in more detail.

To calibrate the methodology for a particular site, it is recommended that
the analyst perform an initial comprehensive reliability run using default
values for all input parameters and subsequently compare the predicted
travel time index (TTI) cumulative distribution to the observed distribution.
This section provides suggestions on how to change calibration parameters
on the basis of the difference between the two TTI distributions.


**Exhibit 25-32: Comprehensive Reliability Calibration Steps**


**Step 1: Gather Input Data**

In this step, all the input data required for a reliability analysis are
gathered. These data include

1. Demand distribution over the reliability reporting period, converted
to monthly and day-of-week demand multipliers;

2. Incident or crash rates and event durations, with the corresponding
speed and capacity adjustment factors;

3. Weather probabilities, with the corresponding speed and capacity
adjustment factors; and

4. Work zone and special event data, with the corresponding speed and
capacity adjustment factors.


Specific details about these input data are provided in Chapter 11,
Freeway Reliability Analysis.


**Step 2: Determine Demand Multipliers**

As mentioned above, the demand level for the seed day is either known
or calibrated at the core freeway facility analysis level. However, in
addition to the seed day, the reliability analysis requires the demand level for
the other days included in the reliability reporting period. Because it is not
feasible to measure demand level for all days, the methodology uses demand
multipliers to convert the seed day demand to demand level for different
days.

Although the demand level of the seed day may be accurately measured,
the seed day may have experienced unusually low or high demand levels. In
that event, the seed day demand either inflates or deflates the demand level
for the other days of the reliability reporting period. In the example shown in
Exhibit 25-33, a high demand level on the seed day causes the resulting TTI
distribution to be consistently shifted to the right compared to the distribution
observed in the field, across the full range of the distribution. Key reliability
performance measures, such as TTImean or TTI95, are also overestimated by
the procedure in the case shown. To fix this problem (i.e., an inflated demand
level for the seed day), the analyst needs to reduce the demand level in the
seed file and make additional runs to determine whether the problem is
resolved.


**Exhibit 25-33: High Demand Level on the Seed Day**


Note also in Exhibit 25-33 that the intercept with the _x_ -axis is the same
for both distributions, suggesting that the free-flow travel time at very low
demands is the same in both cases. If the two distributions do not match at
very low flow rates, this may be an indication that the free-flow speed
calibration step for the core method was not performed correctly.

In contrast, in the example shown in Exhibit 25-34, the predicted TTI
values are consistently lower than the observed values, suggesting that the
seed day has an unusually low demand level. To resolve the problem, the
demand level on the seed day should be increased and additional reliability
runs performed.


**Exhibit 25-34: Low Demand Level on the Seed Day**


Another calibration lever is to change the distribution of the demand
multipliers over the days of the reliability reporting period. This effort can
improve the calibration of the methodology; however, its outcome is harder
to predict. Users should change the distribution only when they have
additional field information about seasonal and daily changes in the demand
level that can bring it closer to reality.

When adjusting the demand level, users should try to bring the estimated
50th percentile TTI value to within 10% of the field-observed value. This is
an iterative process that requires adjusting either the seed day demand level
or the distribution of the demand multipliers, performing an additional
comprehensive reliability run, and comparing the modeled and fieldmeasured 50th percentile TTI values.


**Step 3: Calibrate Incident Probabilities**

When the demand level is calibrated, the predicted and observed TTI
distributions are expected to closely follow each other up to the 50th to 60th
TTI percentiles. However, nonrecurring sources of congestion usually
influence the higher percentiles of the TTI distribution. They may cause a


drift in distributions for higher percentiles, as shown in Exhibit 25-35. The
figure shows a match between the predicted (red) and observed (blue) TTI
distributions, but then suggests an overestimation of TTIs for higher
percentiles with the red curve shifted to the right. As a result, to more
accurately calibrate the comprehensive reliability analysis, the focus should
be on incident and weather events. Incidents are known to have a more
considerable impact on congestion level, and therefore the model is
calibrated for incidents first, followed by weather events.


**Exhibit 25-35: Overestimating the Impacts of Nonrecurring Sources of Congestion**


Incidents can be calibrated by using a number of parameters as listed
below:

1. Probability of incident severity for each month, or crash rate per 100
million vehicle-miles traveled for each month and crash-to-incident
rate and incident severity distribution, depending on the approach
used for scenario generation;

2. Incident duration attributes by severity type (mean, standard
deviation, and distribution);

3. Capacity and speed adjustment factors by severity type; and


4. Demand adjustment factors by severity type.

Incident attributes can be used to address overestimation in the tail of the
predicted TTI distribution and to bring it closer to the observed distribution.
For the example shown in Exhibit 25-35, the predicted and observed TTI
distributions almost match each other up to the 60th TTI percentile,
indicating that the demand level and base congestion level (i.e., recurring
congestion) are calibrated well. After the 60th percentile, the reliability
methodology overestimated TTI values in this case.

To reduce TTI values, the analyst should start by reducing the crash rate
or incident probability. The same effect is expected by reducing the demand
adjustment factor (for incidents). Note that in the case of severe incidents, a
significant reduction in the demand level is expected, as drivers start to
reroute to avoid the congestion. Finally, increasing the capacity and speed
adjustment factors are expected to reduce the impacts of incidents as well.

On the other hand, if the method underestimates TTI values at the tail of
the distribution (see Exhibit 25-36), the user can increase the crash rate,
incident probability, or demand adjustment factor. (Note that the maximum
allowable value for the demand adjustment factor is 1.) In addition, reducing
capacity and speed adjustment factors for incidents is expected to magnify
the impacts of incidents on travel time and consequently increase TTI values.


**Exhibit 25-36: Underestimating the Impacts of Nonrecurring Sources of Congestion**


**Step 4: Calibrate Weather Probabilities**

Similar to incidents, weather events influence the tail of the TTI
distribution, but to a lesser extent. The following calibration parameters are
available:

1. Probability of different weather events by month,

2. Duration of each weather event,

3. Capacity and speed adjustment factors, and

4. Demand adjustment factor.

These calibration parameters are expected to impact the TTI distribution
similarly to those parameters mentioned in Step 3 for incident calibration.
Note that weather information is more likely to be accurate as it is based on
10 years of data, while incident data are more difficult to gather. In addition,
incidents have a more considerable impact on the TTI distribution.
Therefore, as mentioned previously, it is recommended that the methodology
be calibrated first through the demand and incident data, with the analyst


turning to the weather-related parameters only if additional calibration is
required.

For the example shown previously in Exhibit 25-35, the model
overestimated TTI values in the tail of the distribution. The analyst can bring
the two distributions closer to each other by reducing the probability of
different weather events or by reducing their duration. The same effect is
possible by increasing the capacity and speed adjustment factors or by
reducing the demand adjustment factor. Note that in the case of extreme
weather events, a significant reduction in the demand level is expected as
travelers might decide to cancel their trips. However, data on such trends are
very scarce and hard to collect. It is recommended that analysts adjust the
demand adjustment factors only when there is evidence or knowledge of the
trends on the study facility.

On the other hand, when the methodology underestimates TTI values in
the tail of the distribution, as in Exhibit 25-36, the analyst can increase the
probability of weather events or increase their durations. In addition, a
reduction in capacity and speed adjustment factors is expected to move the
distribution to the right.


**Step 5: Validation**

Changing all of the calibration parameters at the same time might lead to
unexpected results. Therefore, the user is encouraged to change only one
parameter at a time, run the comprehensive reliability methodology, plot and
evaluate the new TTI distribution, and only then decide whether and how to
change other parameters. The use of a computational engine makes running
repeated reliability analyses with changing inputs a straightforward process.

The analyst should try to bring at least the predicted 80th and 95th
percentile TTI values within 10% of the field-observed values. Preferably,
additional percentiles should match the field data, although a perfect match
may not be achievable. The collected field data should span the same
reliability reporting period that was selected for the analysis, to ensure that
results are comparable.


**CALIBRATION AT THE RELIABILITY STRATEGY ASSESSMENT**
**LEVEL**

Calibration at the reliability strategy assessment level is only possible
for strategies that have already been implemented in the field. For other
strategies, calibration is not possible, other than based on expert judgment or
comparison to an alternative tools analysis. However, the user can run a set
of sensitivity analyses for each strategy to identify the trends and make sure
that they match expectations. For example, a ramp-metering strategy is
expected to shift the TTI distribution to the left, toward lower TTI values.
The lower the metering rate, the larger the expected shift. If such a trend is
observed, and if its extent is in a reasonable range, one can conclude that
methodology works reasonably.

Similar to the calibration procedure at the comprehensive reliability
level, the analyst must first gather all input data on facility geometry, freeflow speed, and demand level. Note that an important assumption is that the
demand, incident, and weather calibration parameters are already fixed in the
comprehensive reliability calibration step. As a result, the analyst is left with
the remaining calibration parameters that are specific to each scenario.

In general, different scenarios may change a facility’s free-flow speed,
capacity, demand, incident probability, and average incident duration.
Therefore, “scenario-specific” calibration parameters are

1. Speed adjustment factor,

2. Capacity adjustment factor,

3. Metering rate,

4. Demand adjustment factor,

5. Incident probability, and

6. Average incident duration.

It is recommended that the analyst make a reliability strategy assessment
run based on a combination of field measurements and default values, plot
the predicted TTI distribution, and then compare the result to the field
observation. Similar to the comprehensive reliability calibration procedure,
the analyst can then make changes in the calibration parameters to bring the
predicted distribution closer to the observed one.


Based on the modifications that each strategy makes in the freeway
methodology, the user can adjust the corresponding calibration parameters.
Similar to calibrating the comprehensive reliability methodology, increasing
the speed adjustment factor is expected to reduce travel time across the
facility, while reducing it has an opposite effect. Increasing the value of the
capacity adjustment factor is expected to reduce the facility travel time.
Increasing the metering rate will allow more vehicles to enter the mainline
and is expected to increase the facility travel time and perhaps activate
bottlenecks in merge areas. On the other hand, reducing the metering rate is
likely to reduce travel time across the facility and eliminate bottlenecks at
merge areas. Increasing the demand adjustment factor is expected to increase
travel time throughout the facility and shift the TTI distribution toward larger
TTI values, while reducing it has the opposite effect. Increasing the incident
probability is expected to shift the tail of the TTI distribution toward higher
TTI values, while reducing it shifts the tail toward lower values. Finally,
changing the average incident duration is expected to influence the TTI
distribution similarly to incident probability.

The analyst should avoid making several changes in calibration
parameters at the same time, as this may result in changes in TTI distribution
that are hard to explain and may make the calibration procedure more
difficult. Instead, analysts should select one calibration parameter at a time,
make changes, rerun the strategy assessment procedure, plot the TTI
distribution, compare it to the field distribution, and make other changes as
necessary.

The user needs to first identify the main source of difference between the
predicted and field TTI distributions. If a difference between the two
distributions is observed throughout all ranges of TTIs (similar to Exhibit
25-33 and Exhibit 25-34), changing parameters such as the speed adjustment
factor, capacity adjustment factor, demand adjustment factor, and metering
rate is expected to bring the two distributions closer. The analyst should aim
for a maximum of 10% difference between the 50th percentile of the
predicted and observed TTI distributions at this stage.

On the other hand, if the difference between TTI distributions is observed
mostly in the tail of the distribution (similar to Exhibit 25-35 and Exhibit 2536), changing the incident probability and duration is expected to move the
predicted distribution to the right. The analyst should aim for a maximum


10% difference between the 80th and 95th percentiles of the predicted and
observed TTI distributions at this stage as well.


## **9. FREEWAY SCENARIO GENERATION**

**INTRODUCTION**

This section provides details of the freeway scenario generation process.
An overview of this process is provided in Chapter 11, Freeway Reliability
Analysis, and elsewhere ( _17_ ).

Freeway scenario generation utilizes a hybrid process, which includes
deterministic and stochastic methods for modeling traffic demand, weather
events, work zones, and incidents. The freeway reliability methodology uses
a deterministic, calendar-based approach to model traffic demand levels and
scheduled, significant work zone events. It uses a stochastic (Monte Carlo)
approach to assign the occurrence of incident and weather events to
scenarios. The method enumerates the different operational conditions on a
freeway facility on the basis of varying combinations of factors affecting the
facility travel time. Each unique set of operational conditions constitutes a
_scenario_ . A single replication of a scenario represents a unique combination
of a day of week and month of year. The following seven principal stages,
depicted in Exhibit 25-37, are involved in the scenario generation process:

   - Stage 1, based on the user inputs, computes the number of different
demand combinations and the resulting number of scenarios, along
with their probabilities. These values also depend on the duration of
the reliability reporting period.

   - Stage 2 uses local traffic demand data to characterize the demand
levels in the generated scenarios in a deterministic, calendar-based
manner.

   - Stage 3 incorporates scheduled work zones deterministically based
on the calendar.

   - Stage 4 incorporates published local weather event information, and
generates the number and type of weather events, consistent with
local data.


   - Stage 5 randomly assigns the generated weather events in Stage 4 to
the scenarios generated in Stage 1.

   - Stage 6 utilizes the local crash or incident database to generate the
number and severity of incident events, consistent with local data.

   - Stage 7 randomly assigns incidents and their characteristics to each
generated scenario in Stage 1.

The time frame within a given day when the reliability analysis is
performed is called a _study period_ . It consists of several contiguous 15-min
_analysis periods,_ which is the smallest temporal unit of analysis used in
reliability procedures. The smallest spatial unit on the facility is an HCM
analysis segment (see Chapters 12–14). The reliability reporting period is
the time period over which the travel time distribution is generated
(typically, but not necessarily, one year).

Each scenario representing a study period is characterized by a unique
set of segment capacities, demands, free flow speeds, and number of lanes,
for both general purpose and managed lane segments on the freeway facility.
Various scenarios are created by adjusting one or more of the above
parameters. A probability value is associated with each scenario that
represents its likelihood of occurrence. This probability is computed on the
basis of the number of scenarios and replications.


**Exhibit 25-37: Process Flow Overview for Freeway Scenario Generation**


Scenarios are generated in such a manner that the characteristics of the
factors affecting travel time within scenarios best match the input, fieldobserved conditions. For example, the distribution of the number of incidents
generated in various scenarios should yield a distribution similar to that
observed in the field. Exhibit 25-38 depicts such an example, in which the
number of incidents modeled in all scenarios (histogram) is designed to
match field-observed values (curve).


**Exhibit 25-38: Distribution of Number of Incidents in the Scenarios**


Therefore, the process of generating scenarios effectively turns into an
optimization problem. The objective is to maximize the match (or minimize
the difference) between the predicted and field-observed distributions by
assigning appropriate traffic demand levels, weather events, work zones, and
incidents within the different scenarios. Eight distributions are considered in
the scenario generation procedure:

1. Temporal distribution of traffic demand level (typically expressed as
a ratio of scenario demand to AADT),


2. Temporal distribution of weather event frequency (by calendar
month, randomly assigned to scenarios),

3. Distribution of average weather event duration by weather event type
(by calendar month),

4. Temporal distribution of incident event frequency (by calendar
month, weighted in the facility by segment VMT),

5. Distribution of incident severity (user specified),

6. Distribution of incident duration by severity (user specified),

7. Distribution of incident event start time (random), and

8. Spatial distribution of incident events (random).

The scenario generation method attempts to generate scenarios such that
all eight specified distributions match field observations, with consideration
for the need to round to integer values and to the 15-min duration of the
analysis period. Such rounding is not likely to generate any significant
systematic bias in the analysis.


**METHODOLOGY**

The freeway reliability scenario generation methodology consists of 34
steps. Exhibit 25-39 shows the methodology’s process flow. Note that when
managed lanes are present on the facility, the reliability scenarios should
also consider their varying operational characteristics. The methodology
assumes traffic demand levels and weather events affect both general
purpose and managed lane operations simultaneously. However, the
methodology does not account for scheduled work zone events on the
managed lanes. Analysts should repeat Steps 19–34 should they desire to
model incident events on the managed lanes separately. An explanation of
each step in the process flow follows. All variables used in this section are
defined in Section 2.


**Step 1: Prepare Necessary Data for the Reliability Analysis**

In this step, the analyst provides all necessary data for executing the
scenario generation method. The starting point is preparing a complete seed


file describing the facility’s demand and geometry for a single study period.
Developing the seed file is akin to developing a data set for the core
methodology, as described in Chapter 10. In addition, for scenario generation
purposes, additional data must include ( _a_ ) the start and end clock times of the
study period, ( _b_ ) the duration of the reliability reporting period, ( _c_ ) the seed
file date, ( _d_ ) the series of demand multipliers (see Step 4) for each demand
combination, ( _e_ ) the nearest metropolitan area to the facility (for weather
station data), ( _f_ ) the crash or incident rates by month of year on the facility,
and ( _g_ ) other local inputs.


**Step 2: Determine the Number of Demand Combinations**

The freeway scenario generation method defines a demand combination
as the combination of a specific weekday and month of year. Although
demand levels in different demand combinations might be very similar (e.g.,
Tuesday and Wednesday afternoon volumes), the methodology handles them
separately to keep the process simple. For a 1-year, weekday-only analysis,
there are 60 such combinations (5 × 12). The number of demand
combinations is defined by the variable _NDC_ .


**Step 3: Create Scenario Sets and Associate Them with Demand**
**Combinations**

As a default, the methodology creates four scenario _replications_ for each
demand combination. The rationale behind four replications is that each
demand combination usually consists of four or five calendar days. However,
if a short-duration reliability reporting period is considered, the number of
replications must be increased to capture sufficient variability in the travel
time distribution. Typically, however, the default number of scenarios for a
1-year, weekday-only analysis would be 4 × 60 = 240 scenarios. The method
allows the analyst to specify the number of replications per reliability
analysis.


**Exhibit 25-39: Detailed Freeway Scenario Generation Flowchart**


Note: Numbers in brackets are default values.


For each scenario, a set of adjustment factors is created for capacity,
speed, demand, and number of lanes (CAF, SAF, DAF, and NLAF,
respectively). At this point, each scenario contains default values for CAF,
SAF, and DAF (all equal to 1) and NLAF (equal to 0), but the scenarios do


not yet contain any demand, weather, or incident data. _NScen_ represents the
total number of scenarios and is computed as:


**Equation 25-71:**


**Step 4: Assign a Traffic Demand Level to Each Scenario Set**

In this step, a traffic demand level is assigned to each scenario set (i.e.,
the number of replications used per scenario). For this purpose, demand
multipliers, representing the ratio of the traffic demand level in each demand
combination to the AADT are used to generate each scenario demand level.
Because each scenario is associated with a unique demand combination, the
ratio of the scenario demand multiplier to the seed file demand multiplier is
used to determine the scenario demand, as shown in Equation 25-72.


**Equation 25-72:**


where

_DAFs_ ( _tp_, = demand adjustment factor for scenario _s_, period _tp_, and
_seg_ ) segment _seg_ ;

_DM_ ( _Seedtp_ ) = demand multiplier associated with the seed file; and

_DM_ ( _s_ ) = demand multiplier associated with scenario _s_ .


The process to calculate any demand value of any cell in a scenario is to
multiply the cell demand value in the corresponding seed file (for the same
HCM segment and analysis period) with the appropriate DAF, as shown in
Equation 25-72. Note that if the facility contains managed lanes, the traffic
demand level generated in this step will be effective for both the general
purpose and managed lanes.


**Step 5: Calculate Scenario Probabilities**

The probability of a scenario occurrence is strictly a function of the
number of days in the associated demand combination. Note that the
probability of a scenario is fixed at this step and will not be altered in any
subsequent steps. Simply stated, the probability of each scenario does not
change by incorporating weather and incident events. The probability of each
scenario is computed based on Equation 25-73.


**Equation 25-73:**


where

_P_ { _s_ } = probability of scenario _s_,

_DCs_ = demand combination associated with scenario _s,_

_n_ Day _,k_ = number of days in the reliability reporting period associated
with demand combination _k_ (typically four for a 1-year
weekday analysis), and

_NDC_ = number of demand combinations.


After computing each scenario’s probability, the probabilities are
assigned to the scenarios created in Step 3. The probability of a scenario is a
function of the number of days in the associated demand combination, which
is typically four or five for a whole-year analysis. For a typical 1-year,
weekday-only analysis, the probability of each scenario is approximately
1/240 or 4.33%.


**Step 6: Determine Whether All Work Zones Have Been Assigned**

If there are no scheduled work zones during the reliability reporting
period, or if all scheduled work zones have been assigned to scenarios, the
process flow proceeds to Step 10. Otherwise, the process moves to Step 7


and assigns the next work zone. If there are no work zones considered in the
reliability analysis, the process flow proceeds to Step 10.


**Step 7: Calculate Active Work Zone Ratios**

In this step, the parameter _rDC_ is calculated. This parameter is the ratio of
each weekday type in which the work zone is active in a given month to the
total number of each weekday type occurring in a given month. An
unassigned work zone event is selected, and _rDC_ is calculated for each month
in which the work zone is active.


**Step 8: Calculate the Adjusted Number of Replications**

For each affected demand combination in which a work zone is present,

replications of a demand combination for which the work zone is active.


**Equation 25-74:**


**Step 9: Assign the Work Zone to the Work Zone Replications**

For each demand combination of each month in which the work zone is
active, assign the work zone to the adjusted number of replications of each
demand combination (equivalently scenarios) calculated in Step 8.


**Step 10: Group Scenarios by Month**

The attributes of inclement weather events are assumed to vary only by
the month of the year. As such, in Step 10, all scenarios associated with a
given month of year are grouped. Typically, this step involves grouping 20
scenarios (four replications of five weekdays each per month.)


**Step 11: Compute the Expected Frequency of Weather Events**
**by Month**


The method uses the expected frequencies of weather events to create and
characterize weather events. Historical data are used to estimate the
probability, average duration, and standard deviation of duration of different
weather conditions. Weather event likelihoods are reported in timewise
probabilities that were computed for 103 metropolitan areas in the United
States on the basis of 10 years of data. The resulting probability tables are
provided as resource material in the Technical Reference Library in online
HCM Volume 4. A listing of the 97 locations used to create the weather data
is provided in Exhibit 25-40.

Only weather events that reduce capacity by more than 5% are included
in the probability calculations. The average event duration and the standard
deviation for each weather category are calculated by using the 10-year
weather data set for each weather station. The probability of weather event
type _i_ in month _j_ is found from Equation 25-75.


**Equation 25-75:**


where SP indicates study period, and _PW_ { _i_, _j_ } is the probability of
encountering weather type _i_ in month _j_ .


**Exhibit 25-40: Listing of Weather Stations with Available Weather Data**


|# Airport Code City, State|# Airport Code City, State|
|---|---|
|**1**<br>KBHM<br>Birmingham, AL<br>**2**<br>KLIT<br>Little Rock, AR<br>**3**<br>KPHX<br>Phoenix, AZ<br>**4**<br>KTUS<br>Tucson, AZ<br>**5**<br>KBFL<br>Bakersfield, CA<br>**6**<br>KFAT<br>Fresno, CA<br>**7**<br>KLAX<br>Los Angeles, CA<br>**8**<br>KMOD<br>Modesto, CA<br>**9**<br>KCMA<br>Oxnard, CA<br>**10**<br>KROC<br>Riverside, CA<br>**11**<br>KSAN<br>Sacramento, CA|**50**<br>KGSO<br>Greensboro, NC<br>**51**<br>KRIC<br>Raleigh, NC<br>**52**<br>KOMA<br>Omaha, NE<br>**53**<br>KABQ<br>Albuquerque, NM<br>**54**<br>KLAS<br>Las Vegas, NV<br>**55**<br>KALB<br>Albany, NY<br>**56**<br>KBUF<br>Buffalo, NY<br>**57**<br>KLGA<br>New York, NY<br>**58**<br>KPOU<br>Poughkeepsie, NY<br>**59**<br>KSAC<br>Rochester, NY<br>**60**<br>KSYR<br>Syracuse, NY|


Source: Zegeer et al. ( _18_ ).


Equation 25-76 is used to convert those reported probabilities into
rounded expected monthly weather event frequencies.


**Equation 25-76:**


where

_E_ [ _nw_, _j_ ] = expected frequency of weather event _w_ in month _j_,
rounded to the nearest integer;

_P_ t{ _w_, _j_ } = timewise probability of weather type _w_ in month _j_ ;

_DSP_ = duration of study period _SP_ (h);

_NScen,j_ = number of scenarios associated with month _j_ of the
reliability reporting period; and

_E_ 15min[ _Dw_ ] = expected duration of weather event _w_ rounded to the
nearest 15-min increment.


In this step, the _E_ [ _nw_, _j_ ] values for each weather type _w_ are computed in
each month _j_ of the reliability reporting period. Note that the unit of the
expected frequency is _events per total scenario hours in each month_ . Also
note that the minimum value for _E_ 15min[ _Dw_ ] is 0.25 h.

For example, if the study period is 5 h, if the probability of light rain
during that month and time period (typically associated with about 20
scenarios) is 0.10, and if the average light rain event lasts 1 h, then the
expected number of light rain events in that month is (0.1 × 5 × 20)/1, which
rounds to 10 light rain weather events in that month, or 10 h of light rain in
the month.


**Step 12: Select a Month with Unassigned Weather Events**

The process of assigning weather events in a month is independent of
other months in the reliability reporting period. The process is carried out on


a monthly basis. For this purpose, one month from the reliability reporting
period without an assigned weather event is selected in the next steps.


**Step 13: Update the List of Weather Events**

In this step, the list of weather events is updated. That is, the weather
events associated with the current month will have their characteristics
(durations, CAFs, and SAFs) assigned.


**Step 14: Assign Weather Events and Start Times to Scenarios**

In this step, a weather event that was updated in the list of weather events
in Step 13 is selected and randomly assigned to a scenario in the current
month. The assignment of weather events to scenarios is carried out
consistent with the relative scenario probabilities. In addition, a start time is
randomly assigned to the selected weather event from the list of weather
events. Because actual data on the start time of weather events are lacking,
those are assigned randomly based on a uniform distribution.


**Step 15: Identify Overlaps Between Weather Events in a Single**
**Scenario**

This step ensures there will be no temporal overlap between two
weather events within a single scenario. Possible overlaps between weather
events are checked, and if they exist, then Step 16 is executed. Otherwise, the
process moves to Step 17.


**Step 16: Undo the Most Recent Weather Event Assignment**

If there is an overlap between weather events, the most recent weather
assignment is undone. The process then goes back to Step 14 to reassign a
scenario and a start time for the weather event.


**Step 17: Check for Unassigned Weather Events in the Current**
**Month**

This step checks that all weather events present in the list of weather
events have been assigned. If one or more unassigned weather events exist


for the current month, the process returns to Step 14 to select another
unassigned weather event.


**Step 18: Check for Unassigned Weather Events in All Months**

Once all weather events have been assigned to scenarios across all
months in the reliability reporting period, the methodology proceeds to the
incident modeling stage. Otherwise, the process returns to Step 12 to select
another month from the reliability reporting period to have its weather events
modeled in the associated scenarios.


**Step 19: Select a Month with Unassigned Incidents**

The methodology allows the user to directly enter monthly incident
occurrences on a given facility during the study period into the procedure,
should these values be available. Optimally, the distribution of incident
durations, the start times, and the distribution of incidents by severity (e.g.,
number of lanes closed) could also be entered directly from a local incident
database.

However, in most cases (including predictive reliability applications),
these data will not be available, and incident events will need to be
estimated from incident or crash rates (which vary by month and traffic
demand levels). The methodology accounts for the correlation between
incident and crash-only rates. Because the method attempts to generate the
number of incident events based on their distributions, a high number of
incidents could be assigned to a scenario that is associated with a low traffic
demand level. The average traffic demand level for each month is therefore
computed and used to characterize the incident events within scenarios in
each month. Incident events are assigned to different months of the reliability
reporting period independently. Therefore, a month from the reliability
reporting period without any assigned incidents is first selected in the next
steps.


**Step 20: Compute the Expected Incident Frequency**

The expected frequency of all incidents on the facility per study period in
a given month _j_ is computed with Equation 25-77.


**Equation 25-77:**


where

_nj_ = expected frequency of all incidents in the study period for
month _j_, rounded to the nearest integer;

_IRj_ = incident rate per 100 million VMT in month _j_ ; and

_VMTj_ = average vehicle miles traveled for scenarios in month _j_, after
adjusting the demand in the base scenario with the appropriate
demand multipliers and multiplying by the facility length in
miles.


If _IRj_ is not locally available, Equation 25-78 can be used to estimate it.


**Equation 25-78:**


where _CRj_ is the local facilitywide crash rate per 100 million VMT in month
_j_ and _ICR_ is the local incident-to-crash ratio. In the absence of other data, a
national default value for _ICR_ is 4.9.

When the crash rate is not available locally, the Highway Economic
Requirements System (HERS) model can be used to estimate it ( _19_ ).
Agencies may also use other predictive models such as the _Highway Safety_
_Manual_ ( _20_ ). The crash or incident rate is estimated per 100 million VMT.
The HERS model uses Equation 25-79 to estimate the crash rate.


**Equation 25-79:**


where _CR_ is the crash rate per 100 million VMT, _ACR_ is the facility AADT
divided by its two-way hourly capacity, and _LW_ is the lane width in feet.


**Step 21: Generate a Set of Incident Frequencies**

The distribution of the number of incidents in a study period can be
characterized by a Poisson distribution. Assume there are _NScen,j_ scenarios
(typically 20) associated with the current month _j_ . Then, on average, _nj_ ×
_NScen,j_ incidents (rounded to the nearest integer) need be to generated and
assigned to scenarios. Therefore, a set of _NScen,j_ numbers should be
generated that best matches a Poisson distribution with a mean value of _nj_,
per Equation 25-80.

For this purpose, an adjustment parameter _δ_ 1 is defined. By solving
Equation 25-80, the frequency of incidents for a set of _NScen,j_ scenarios can
be computed, following the Poisson distribution. The values of the
adjustment parameter usually hover around 1 and are estimated from the
equality.


**Equation 25-80:**


where _ninc_ is the number of incidents and other variables are as defined
previously. Subsequently, the number of scenarios that are assigned _k_
incidents ( _k_ = 0 (() is determined by Equation 25-81.


**Equation 25-81:**


where all variables are as defined previously. By setting different _k_ -values in
the above equation, a set of monthly incident frequencies will be generated in
this step.


**Step 22: Assign Incidents to Scenarios**

The incidents generated in Step 21 are randomly assigned to the
scenarios associated with the current month. A random number is drawn with
respect to scenario probabilities to determine the assigned scenario number.


**Step 23: Update the List of Incident Events**

The list of incident events is updated after the incident frequencies are
generated. This list holds information for each incident event in the entire
reliability analysis. The associated incident event information includes the
assigned scenario number, calendar month, incident duration, incident impact
factors (e.g., CAF, SAF), incident segment location, and incident start time.


**Step 24: Check for Unassigned Incidents**

This step ensures that incident event frequencies are generated and
assigned to scenarios for all months in the reliability reporting period. Once
incidents in all months have been processed in Steps 20–23, the scenario
generation process continues to Step 25.


**Step 25: Generate Incident Severities for Each Incident Event**

A set of incident severities is generated for the entire set of incidents
developed in Step 21. Note that this step is not carried out on a monthly
basis. The distribution of incident severities must be known a priori for
incorporation in the methodology. This distribution is defined by ( _i_ ), which
is assumed to be homogeneous across the facility and different demand
levels.

Agencies can estimate this distribution by analyzing their incident logs or
they can use national default values. Equation 25-82 gives the definition of
( _i_ ) as a discrete distribution, where _i_ denotes the incident severity type (e.g.,
_i_ = 1 is a shoulder closure, and _i_ = 5 is a four-lane closure).


**Equation 25-82:**


Suppose a total of _NScen,Inc_ incidents was generated in Steps 19–24. To
generate incident severities, an adjustment parameter _δ_ 2 is defined. By
solving Equation 25-83, incident severities for all incidents in the list of
incident events will be estimated that will follow the prespecified ( _i_ )
distribution.


**Equation 25-83:**


where all variables are as previously defined. The adjustment parameter is
determined with Equation 25-83, and the number of scenarios that are
assigned incident severity type _i_ is determined by Equation 25-84.


**Equation 25-84:**


where all variables are as previously defined.

The distribution of incident severity ( _i_ ) is shown in Equation 25-85.
These values are based on national default values ( _18_ ).


**Equation 25-85:**


_i_ = 1 (shoulder closed)
_i_ = 2 (one lane closed)
_i_ = 3 (two lanes closed)
_i_ = 4 (three lanes closed)
_i_ = 5 (four or more lanes closed)


**Step 26: Assign Incident Severity Type**

The incident severities generated in Step 25 are randomly assigned to the
incidents in the list of incident events.


**Step 27: Generate Incident Durations by Incident Severity Type**

The duration of each incident severity type is assumed to follow a
lognormal distribution ( _15_ ). Exhibit 25-41 shows default parameters for the
incident duration distribution developed through research ( _18_ ).


**Exhibit 25-41: Incident Duration Distribution Parameters in Minutes**







|Statistics|Col2|No. of Lanes Closed<br>Shoulder 1 2 3 or more|
|---|---|---|
|Range<br>Average<br>Median<br>Standard devi|ation|8.7–58<br>16–58.2<br>30.5–66.9<br>36–93.3<br>34.0<br>34.6<br>53.6<br>69.6<br>36.5<br>32.6<br>60.1<br>67.9<br>15.1<br>13.8<br>13.9<br>21.9|


Because _NInc,i_ incidents are associated with severity _i_, a set of _NInc,i_
numbers can be generated that best matches a lognormal distribution of
incident durations. For this purpose, an adjustment parameter _δ_ 3 is defined,
as shown in Equation 25-86.


**Equation 25-86:**


where _IncDur_ is the incident duration in minutes, _Inc_ Type is the incident
severity type (1–5, as listed in Equation 25-85), and other variables are as
defined previously.

By solving Equation 25-86, the adjustment parameter is determined. The
number of scenarios that are assigned an incident duration _t_ are then
determined by Equation 25-87.


**Equation 25-87:**

Number of scenarios assigned incident


where all variables are as defined previously.

By inserting different _t-_ values in Equation 25-87, a set of incident
durations for each incident severity type will be generated.


**Step 28: Randomly Assign Incident Durations by Severity**

The incident durations generated in Step 27 are randomly assigned to the
incidents in the list of incident events on the basis of the incident severity.


**Step 29: Generate the Distribution of Incident Start Times and**
**Locations**

In this step, the distribution of each incident start time and location is
assigned based on Step 20, with the likelihood of having an incident on a
segment in a given analysis period being correlated to the segment VMT. The
distribution of incident start times will coincide with the distribution of
facility VMT across all analysis periods. Further, the distribution of the
location of an incident will be similarly tied to the distribution of VMT for
each segment across the study period. Since _VMTseg,p_ represents the VMT on


segment _seg_ during analysis period _p_ in the seed file, the distribution of the
incident locations will be determined by Equation 25-88.


**Equation 25-88:**


where _Location_ is the segment in which the incident occurs.

In a similar manner, the distribution of the incident start time will be
determined by Equation 25-89.


**Equation 25-89:**


where _StartTime_ is the analysis period in which the incident starts.


**Step 30: Generate Incident Start Times and Locations for All**
**Incidents**

Assuming there are _NScen,Inc_ incidents in the list of incident events, two
sets of _NScen,Inc_ numbers should be generated that best match the incident
start time and location distributions. For this purpose, two adjustment
variables, _δ_ 4 and _δ_ 5, are defined by Equation 25-90 and Equation 25-91,
respectively.


**Equation 25-90:**


**Equation 25-91:**


By solving Equation 25-90 and Equation 25-91, the adjustment
parameters are determined. The number of incidents that are assigned to any
segment _seg_ are then determined from Equation 25-92.


**Equation 25-92:**

Number of incidents assigned to segment


Finally, the number of incidents that are assigned a starting time (analysis
period _p_ ) is determined from Equation 25-93.


**Equation 25-93:**


By inserting different _seg_ and _p_ values in the above equations, a set of
incident locations and start times will be generated.


**Step 31: Assign Start Times and Locations to Incidents**

In this step, an incident from the list of incident events is selected whose
start time and location have not been assigned. A start time and location
already generated in Step 30 are randomly assigned to the selected incident.


**Step 32: Check for Overlap with Previously Assigned Incidents**


This step checks if there is any overlap between other incident events for
which the start time and location have been assigned in the list of incident
events. If there is an overlap, the process proceeds to Step 33. Otherwise, it
proceeds to Step 34.


**Step 33: Undo the Previous Start Time and Location Assignment**

This step undoes the previous start time and location assignment from
Step 31 that led to the identification of a conflict in the list of incident events
in Step 32.


**Step 34: Check Whether All Incident Start Times and Locations**
**Have Been Assigned**

If there are incidents in the list of incident events that have not been
assigned a start time and location, the process returns to Step 31 for further
assignment. Otherwise, all the incidents in the list of incident events have
been fully described and are ready to be modeled in the scenarios.


## **10. COMPUTATIONAL ENGINE OVERVIEW**

The FREEVAL-2015E computational engine is written in the Java
programming language. Java is a free, open source, object-oriented
programming language that is highly portable and will run on almost all
platforms. Unlike procedural languages, which largely consist of code
broken up into subroutines, object-oriented languages require that the code
be expressed in terms of _objects_ . These objects have functions that either
operate on the data associated with them or on other objects. In Java, groups
of objects are called _classes_ . Classes are then grouped into _packages_, which
seek to provide organization based on some shared purpose or similarity.

The computational engine consists of nine packages, each of which
contains a group of classes specific to a certain aspect of the HCM analysis.
The main package contains the two most important classes for the
methodology. First, the Seed class contains all input data for the freeway
facility (e.g., freeway geometry, demand) and is the backbone of the engine.
Once the analysis has been run, the Seed class will also contain all output
performance measures. Further, any reliability or ATDM analysis performed
will use Seed as the basis for its analysis.

The second class in the main package is the GPMLSegment class. This
class is used to represent the segments of the freeway facility (general
purpose or managed lane), and contains the code for both the undersaturated
and oversaturated computational modules. Much of this code is an exact
translation of the HCM methodology, with differences only occurring when it
was necessary to either improve the performance of the code, or to match
Java programming conventions. An example of a difference is that some
variable values may not be explicitly stored but rather are calculated only as
needed.

The other eight packages build on these two main classes. Four of the
packages consist of “helper” functions that are used throughout the code.
These helper classes provide functionality ranging from general input-output
actions, such as opening and saving files, to more specific purposes, such as


creating facility output summaries and specifying parameters for rampmetering methodologies. The final four packages relate to reliability and
ATDM analysis. These packages contain the reliability scenario generator, as
well as many additional data structures to facilitate data input for both
reliability and ATDM analysis.

The Java programming language provides the integrated ability to
generate its own documentation. Developers simply provide descriptions of
classes, functions, and variables throughout the code, and Java compiles
them into a set of documentation referred to as a “Javadoc.” This Javadoc
follows the format of the official documentation of the language, thus
allowing it to be easily understood and used by anyone familiar with the
language. This documentation has been generated and is packaged with the
computational engine. A user guide for the graphical user interface version of
the engine is available to provide guidance on its use. These items can be
found in the Technical Reference Library in online HCM Volume 4.


## **11. EXAMPLE PROBLEMS**

This section presents eleven example problems illustrating the evaluation
of freeway facilities using the core methodology, the reliability methodology,
and the ATDM methodology. Exhibit 25-42 presents a list of these problems.


**Exhibit 25-42: List of Example Problems**


**Example**
**Problem** **Description** **Application**

Operational
1 Evaluation of an undersaturated facility
analysis

Operational
2 Evaluation of an oversaturated facility
analysis

Operational
3 Capacity improvements to an oversaturated facility
analysis

Operational
4 Evaluation of an undersaturated facility with a work zone
analysis

Evaluation of an oversaturated facility with a managed Operational
5
lane analysis
6 Planning-level analysis of a freeway facility Planning analysis
7 Reliability evaluation of an existing freeway facility Reliability analysis
8 Reliability analysis with geometric improvements Reliability analysis
9 Evaluation of incident management ATDM analysis
10 Planning-level reliability analysis Planning analysis

Estimating freeway composite grade operations with Specialized truck
11
the mixed-flow model analysis


**EXAMPLE PROBLEM 1: EVALUATION OF AN**
**UNDERSATURATED FACILITY**


**The Facility**

The subject of this operational analysis is a 6-mi-long urban freeway
facility that is composed of 11 individual analysis segments, as shown in
Exhibit 25-43.


**Exhibit 25-43: Example Problem 1: Freeway Facility**


The facility has three on-ramps and three off-ramps. Geometric details
are given in Exhibit 25-44.


**Exhibit 25-44: Example Problem 1: Geometry of Directional Freeway Facility**


**Segment No.** **1** **2** **3** **4** **5** **6** **7** **8** **9** **10** **11**

B or
Segment type B ONR B OFR B B ONR R OFR B
W

Segment
5,280 1,500 2,280 1,500 5,280 2,640 5,280 1,140 360 1,140 5,280
length (ft)

No. of lanes 3 3 3 3 3 4 3 3 3 3 3


Notes: B = basic freeway segment; W = weaving segment; ONR = on-ramp (merge)
segment; OFR = off-ramp (diverge) segment; R = overlapping ramp segment.


The on- and off-ramps in Segment 6 are connected by an auxiliary lane,
and the segment may therefore operate as a weaving segment, depending on
traffic patterns. The separation of the on-ramp in Segment 8 and the off-ramp
in Segment 10 is less than 3,000 ft. Because the ramp influence area of onramps and off-ramps is 1,500 ft, according to Chapter 14, the segment
affected by both ramps is analyzed as a separate overlapping ramp segment
(Segment 9), labeled “R” in Exhibit 25-44.

The analysis question at hand is the following: What is the operational
performance and LOS of the directional freeway facility shown in Exhibit
25-43?


**The Facts**

In addition to the information contained in Exhibit 25-43 and Exhibit 2544, the following characteristics of the freeway facility are known:

SUTs and buses = 1.25% (all movements);


TTs = 1.00% (all movements);

Driver population = regular commuters;

_FFS_ = 60 mi/h (all mainline segments);

Ramp _FFS_ = 40 mi/h (all ramps);

Acceleration lane length = 500 ft (all ramps);

Deceleration lane length = 500 ft (all ramps);

_D_ jam = 190 pc/mi/ln;

_cIFL_ = 2,300 pc/h/ln (for _FFS_ = 60 mi/h);

_Ls_ = 1,640 ft (for Weaving Segment 6);

Total ramp density _TRD_ = 1.0 ramp/mi;

Terrain = level; and

Analysis duration = 75 min (divided into five 15-min intervals).


A queue discharge capacity drop of 7% is assumed.


**Comments**

The facility was divided into analysis segments on the basis of the
guidance given in Chapter 10, Freeway Facilities Core Methodology. The
facility shown in Exhibit 25-43 depicts seven freeway _sections_ (measured
between ramps) that are divided into 11 analysis _segments._ The facility
contains each of the possible segment types for illustrative purposes,
including basic segment (B), weaving segment (W), merge segment (ONR),
diverge segment (OFR), and overlapping ramp segment (R). The input data
contain the required information needed for each of the segment
methodologies.

The classification of the weave in Segment 6 is preliminary until it is
determined whether the segment operates as a weave. For this purpose, the
short length must be compared with the maximum length for weaving analysis
to determine whether the Chapter 13, Freeway Weaving Segments, or the
Chapter 12, Basic Freeway and Multilane Highway Segments, methodology
is applicable. The short length of the weaving segment used for calculation is


shorter than the weaving influence area over which the calculated speed and
density measures are applied.

Chapter 12 must be consulted to find appropriate values for the heavyvehicle adjustment factor _fHV_ . The computational engine automatically
determines these adjustment factors for general terrain conditions, but user
input is needed for specific upgrades and composite grades.

All input parameters have been specified, so default values are not
needed. Fifteen-minute demand flow rates are given in vehicles per hour
under prevailing conditions. These demands must be converted to passenger
cars per hour under equivalent ideal conditions for use in the parts of the
methodology related to segment LOS estimation. Details of the steps of the
methodology follow.


**Step A-1: Define Study Scope**

In this initial step, the analyst defines the spatial extent of the facility
(start and end points, total length) and the temporal extent of the analysis
(number of 15-min analysis periods). The analyst should further decide
which study extensions (if any) apply to the analysis (i.e., managed lanes,
reliability, ATDM).

According to the inputs provided in the example, the number of analysis
periods is five and the facility has 11 segments. The analysis does not
involve a methodological extension.


**Step A-2: Divide Facility into Sections and Segments**

In this step, the analyst first defines the number of sections from gore
point to gore point along the selected facility. These gore-to-gore sections
are more consistent with modern freeway performance databases than HCM
segments, and this consistency is critical for calibrating and validating the
freeway facility. The analyst later divides sections into HCM segments
(basic, merge, diverge, weave, overlapping ramp, or managed lane segment)
as described in Chapter 10. The subject facility has already been segmented
as shown in Exhibit 25-43.


**Step A-3: Input Data**


Data concerning demand, geometry, and other data are specified in this
step. As the methodology builds on segment analysis, all data for each
segment and each analysis period must be provided. Traffic demand inputs
for all 11 segments and five analysis periods are given in Exhibit 25-45.


**Exhibit 25-45: Example Problem 1: Demand Inputs**







|Analysis<br>Period (15 Entering Flow<br>min) Rate (veh/h)|Ramp Flow Rates by Analysis Period (veh/h)<br>ONR1 ONR2a ONR3 OFR1 OFR2 OFR3|Exiting<br>Flow<br>Rate<br>(veh/h)|
|---|---|---|
|1<br>4,505<br>2<br>4,955<br>3<br>5,225<br>4<br>4,685<br>5<br>3,785|450<br>540(50)<br>450<br>270<br>360<br>270<br>540<br>720(100)<br>540<br>360<br>360<br>270<br>630<br>810(150)<br>630<br>270<br>360<br>450<br>360<br>360(80)<br>450<br>270<br>360<br>270<br>180<br>270(50)<br>270<br>270<br>180<br>180|5,045<br>5,765<br>6,215<br>4,955<br>3,875|


Note: _a_ Numbers in parentheses indicate ONR-2 to OFR-2 demand flow rates in Weaving
Segment 6.


The volumes in Exhibit 25-45 represent the 15-min demand flow rates on
the facility as determined from field observations or other sources. The
actual volume served in each segment will be determined by the
methodology. The demand flows are given for the extended time–space
domain, consistent with the recommendations in Chapter 10. Peaking occurs
in the third 15-min period. Because inputs are in the form of 15-min flow
rates, no peak hour factor adjustment is necessary. Additional geometric and
traffic-related inputs are as specified in Exhibit 25-44 and the Facts section
of the problem statement.


**Step A-4: Balance Demands**

The traffic flows in Exhibit 25-45 are already given in the form of actual
demands. Therefore, balancing demand is not necessary.


**Step A-5: Identify Global Parameters**

Global inputs are jam density and queue discharge capacity drop. Values
for both parameters are given in the example problem’s Facts section.


**Step A-6: Code Base Facility**

Step 6 is the first step requiring the use of a computational engine or
software. Data input needs for the computational engine include all items
collected or estimated in the previous steps. These data generally need to be
entered for each segment and each analysis period, making this one of the
most time-consuming steps in the analysis.


**Step A-7: Compute Segment Capacities**

Segment capacities are determined by using the methodologies of Chapter
12 for basic freeway segments, Chapter 13 for weaving segments, and
Chapter 14 for merge and diverge segments. The resulting capacities are
shown in Exhibit 25-46. Because the capacity of a weaving segment depends
on traffic patterns, including the weaving ratio, it varies by analysis period.
The remaining segment capacities are constant in all five time intervals. The
capacities for Segments 1–5 and 7–11 are the same because the segments
have the same basic cross section. The units shown are in vehicles per hour.


**Exhibit 25-46: Example Problem 1: Segment Capacities**



**Analysis**
**Period**



**Capacities (veh/h) by Segment**
**1** **2** **3** **4** **5** **6** **7** **8** **9** **10** **11**



1



1 8,273

2 8,281
3 6,748 6,748 6,748 6,748 6,748 8,323 6,748 6,748 6,748 6,748 6,748
4 8,403
5 8,463



6,748 6,748 6,748



**Step A-8: Calibrate with Adjustment Factors**

This step allows the analyst to adjust demands, capacities, and FFSs for
the purpose of calibration. The demand adjustment factor (DAF), capacity
adjustment factor (CAF), and speed adjustment factor (SAF) can be modified
for each segment and each analysis period. There is no adjustment needed for
the subject facility according to the problem statement.


**Step A-9: Adjust Managed Lane Cross Weave**


This step is only required for facilities with managed lanes. The subject
facility does not have a managed lane; therefore, this step is not required.


**Step A-10: Compute Demand-to-Capacity Ratios**

The demand-to-capacity ratios in Exhibit 25-47 are calculated from the
demand flows in Exhibit 25-45 and the segment capacities in Exhibit 25-46.


**Exhibit 25-47: Example Problem 1: Segment Demand-to-Capacity Ratios**







|Analysis Period|Demand-to-Capacity Ratios by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|0.67<br>0.73<br>0.73<br>0.73<br>0.69<br>0.63<br>0.72<br>0.79<br>0.79<br>0.79<br>0.75<br>0.73<br>0.81<br>0.81<br>0.81<br>0.76<br>0.71<br>0.81<br>0.89<br>0.89<br>0.89<br>0.85<br>0.77<br>0.87<br>0.87<br>0.87<br>0.83<br>0.77<br>0.89<br>0.99<br>0.99<br>0.99<br>0.92<br>0.69<br>0.75<br>0.75<br>0.75<br>0.71<br>0.61<br>0.71<br>0.77<br>0.77<br>0.77<br>0.73<br>0.56<br>0.59<br>0.59<br>0.59<br>0.55<br>0.47<br>0.56<br>0.60<br>0.60<br>0.60<br>0.57|


The computed demand-to-capacity ratio matrix in Exhibit 25-47 shows
no segments with a _vd/c_ ratio greater than 1.0 in any time interval.
Consequently, the facility is categorized as _globally undersaturated_, and the
analysis proceeds with computing the undersaturated service measures in
Step A-11. Further, it is expected that no queuing will occur on the facility
and that the volume served in each segment is identical to the input demand
flows. Consequently, the matrix of volume-to-capacity ratios would be
identical to the demand-to-capacity ratios in Exhibit 25-47. The resulting
matrix of volumes served by segment and time interval is shown in Exhibit
25-48.


**Exhibit 25-48: Example Problem 1: Volume-Served Matrix**








|Analysis<br>Period|Volumes Served (veh/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|4,505 4,955 4,955 4,955 4,685 5,225 4,865 5,315 5,315 5,315 5,045<br>4,955 5,495 5,495 5,495 5,135 5,855 5,495 6,035 6,035 6,035 5,765<br>5,225 5,855 5,855 5,855 5,585 6,395 6,035 6,665 6,665 6,665 6,215<br>4,685 5,045 5,045 5,045 4,775 5,135 4,775 5,225 5,225 5,225 4,955<br>3,785 3,965 3,965 3,965 3,695 3,965 3,785 4,055 4,055 4,055 3,875|


**Step A-11: Compute Undersaturated Segment Service Measures**

Because the facility is globally undersaturated, the methodology proceeds
to calculate service measures for each segment and each analysis period,
starting with the first segment in Analysis Period 1. The computational
details for each segment type are exactly as described in Chapters 12, 13,
and 14. The weaving methodology in Chapter 13 checks whether the weaving
short length _LS_ is less than or equal to the maximum weaving length _Lmax_ . It is
assumed, for any time interval where _LS_ is longer than or equal to _Lmax_, that
the weaving segment will operate as a basic freeway segment.

The basic performance measures computed for each segment and each
analysis period are the segment speed (Exhibit 25-49), density (Exhibit 2550), and LOS (Exhibit 25-51).


**Exhibit 25-49: Example Problem 1: Speed Matrix**







|Analysis Period|Speed (mi/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|60.0<br>53.9<br>59.7<br>56.1<br>60.0<br>48.0<br>59.9<br>53.4<br>53.4<br>56.0<br>59.7<br>59.9<br>53.2<br>58.6<br>55.8<br>59.6<br>46.8<br>58.6<br>52.3<br>52.3<br>55.7<br>57.6<br>59.4<br>52.6<br>57.2<br>55.7<br>58.3<br>46.2<br>56.2<br>50.6<br>50.6<br>51.8<br>55.1<br>60.0<br>53.8<br>59.7<br>56.1<br>60.0<br>49.7<br>60.0<br>53.6<br>53.6<br>56.0<br>59.9<br>60.0<br>54.9<br>59.8<br>56.3<br>60.0<br>52.5<br>60.0<br>54.8<br>54.8<br>56.5<br>60.0|


**Exhibit 25-50: Example Problem 1: Density Matrix**







|Analysis Period|Density (veh/mi/ln) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|25.0<br>30.6<br>27.6<br>29.4<br>26.0<br>27.2<br>27.1<br>33.2<br>33.2<br>31.6<br>28.1<br>27.6<br>34.5<br>31.2<br>32.8<br>28.7<br>31.3<br>31.2<br>38.5<br>38.5<br>36.1<br>33.4<br>29.3<br>37.1<br>34.1<br>35.0<br>31.9<br>34.6<br>35.8<br>43.9<br>43.9<br>42.9<br>37.6<br>26.0<br>31.3<br>28.1<br>30.0<br>26.5<br>25.8<br>26.5<br>32.5<br>32.5<br>31.1<br>27.6<br>21.0<br>24.1<br>22.0<br>23.5<br>20.5<br>18.9<br>21.0<br>24.7<br>24.7<br>23.9<br>21.5|


**Exhibit 25-51: Example Problem 1: LOS Matrix**








|Analysis Period|LOS by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1|C<br>C<br>D<br>C<br>D<br>C<br>D<br>D<br>D<br>D<br>D|


**Step A-13: Apply Managed Lane Adjacent Friction Factor**

This step is only required for facilities with managed lanes.


**Step A-14: Compute Lane Group Performance**

This step is only required for facilities with managed lanes.


**Step A-15: Compute Freeway Facility Service Performance**
**Measures by Time Interval**

In this analysis step, facilitywide performance measures are calculated
for each analysis period. Example calculations are provided for the first
analysis period only; summary results are shown for all five analysis
periods.

First, the facility space mean speed _SMS_ is calculated for analysis period
_p_ = 1 from the 11 individual segment flows _SF_ ( _i_, _p_ ), segment lengths _L_ ( _i_ ),
and space mean speeds in each segment and analysis period _U_ ( _i_, _p_ ).


=(4,505 × 5,280) + (4,955 × 1,500) + (4,955 × 2,280) + (4,955 ×
1,500) + (4,685 × 5,280) + (5,225 × 2,640) + (4,865 × 5,280) +
(5,315 × 1,140) + (5,315 × 360) + (5,315 × 1,140) + (5,045 ×
5,280)


= 154,836,000 veh-ft


=(4,505 × 5,280 / 60.00) + (4,955 × 1,500 / 53.90) + (4,955 ×
2,280 / 59.70) + (4,955 × 1,500 / 56.10) + (4,685 × 5,280 /
60.00) + (5,225 × 2,640 / 48.00) + (4,865 × 5,280/ 59.90) +


(5,315 × 1,140/ 53.40) + (5,315 × 360 / 53.40) + (5,315 ×
1,140 / 56.00) + (5,045 × 5,280 / 59.70)
= 2,688,234 veh-ft/mi/h


Second, the average facility density is calculated for Analysis Period 1
from the individual segment densities _K_, segment lengths _L_, and number of
vehicles in each segment _N_ .


= (25.0 × 5,280 × 3) + (30.6 × 1,500 × 3) + (27.6 ×
2,280 × 3) + (29.4 × 1,500 × 3) + (26.0 × 5,280 × 3)
+ (27.2 × 2,640 × 4) + (27.1 × 5,280 × 3) + (33.2 ×
1,140 × 3) + (33.2 × 360 × 3) + (31.6 × 1,140 × 3) +
(28.1 × 5,280 × 3)
= 2,685,696 (veh/mi/ln)(ln-ft)


=(5,280 × 3) + (1,500 × 3) + (2,280 × 3) + (1,500 × 3) + (5,280 ×
3) + (2,640 × 4) + (5,280 × 3) + (1,140 × 3) + (360 × 3) + (1,140
× 3) + (5,280 × 3)
= 97,680 ln-ft


These calculations are repeated for all five analysis periods. The overall
space mean speed across all analysis periods is calculated as follows:


The overall average density across all analysis periods is calculated as
follows:


The resulting performance and service measures for Analysis Periods 1–
5 and the facility totals are shown in Exhibit 25-52.


**Exhibit 25-52: Example Problem 1: Facility Performance Measure Summary**







|Analysis Period|Performance Measure<br>Space Mean Speed (mi/h) Average Density (veh/mi/ln)|LOS|
|---|---|---|
|1<br>2<br>3<br>4<br>5|57.6<br>27.5<br>56.6<br>31.3<br>55.0<br>34.8<br>57.9<br>27.5<br>58.4<br>21.4|D<br>D<br>E<br>D<br>C|
|**Total**|**56.9**<br>**28.4**|**—**|


**Step A-16: Aggregate to Section Level and Validate Against Field**
**Data**

This step is used to validate the analysis and is performed only when
field data are available.


**Step A-17: Estimate LOS and Report Performance Measures for**
**Lane Groups and Facility**

The LOS for each time interval is determined directly from the average
density for each time interval by using Exhibit 10-7. No LOS is defined for
the average across all time intervals.


**Discussion**


This facility turned out to be globally undersaturated. Consequently, the
facility-aggregated performance measures could be calculated directly from
the individual segment performance measures. An assessment of the segment
service measures across the time–space domain can begin to highlight areas
of potential congestion. Visually, this process can be facilitated by plotting
the _vd/c_, _va/c_, speed, or density matrices in contour plots.


**EXAMPLE PROBLEM 2: EVALUATION OF AN OVERSATURATED**
**FACILITY**


**The Facility**

The facility used in Example Problem 2 is identical to the one in
Example Problem 1, which is shown in Exhibit 25-43 and Exhibit 25-44.


**The Facts**

In addition to the information in Exhibit 25-43 and Exhibit 25-44, the
following characteristics of the freeway facility are known:

SUTs and = 1.25% (all movements);
buses

TTs = 1.00% (all movements);

Driver = regular commuters;
population

_FFS_ = 60 mi/h (all mainline segments);

Ramp _FFS_ = 40 mi/h (all ramps);

Acceleration = 500 ft (all ramps);
lane length

Deceleration = 500 ft (all ramps);
lane length

_D_ jam = 190 pc/mi/ln;

_cIFL_ = 2,300 pc/h/ln (for _FFS_ = 60 mi/h);

_Ls_ = 1,640 ft (for Weaving Segment 6);


_TRD_ = 1.0 ramp/mi;

Terrain = level;

Analysis = 75 min (divided into five 15-min analysis periods); and
duration

Demand = +11% increase in demand volumes across all segments
adjustment and analysis periods relative to Example Problem 1.


As before, a queue discharge capacity drop of 7% is assumed.


**Comments**

The facility and all geometric inputs are identical to Example Problem 1.
The same general comments apply. The results of Example Problem 1
suggested a globally undersaturated facility, but some segments were close to
their capacity ( _vd/c_ ratios approaching 1.0). In the second example, a
facilitywide demand increase of 11% is applied to all segments and all
analysis periods. Consequently, it is expected parts of the facility may
become oversaturated and queues may form on the facility.


**Step A-1: Define Study Scope**

Similar to Example Problem 1, there are five analysis periods and the
facility has 11 segments. The analysis does not include any extensions such
as managed lanes, reliability, ATDM, or work zones.


**Step A-2: Divide Facility into Sections and Segments**

The subject facility segmentation is given in Exhibit 25-43. Therefore,
there is no need to go through the segmentation process.


**Step A-3: Input Data**

The revised traffic demand inputs for all 11 segments and five analysis
periods are shown in Exhibit 25-53.


**Exhibit 25-53: Example Problem 2: Demand Inputs**


|Analysis Entering Flow<br>Period Rate<br>(15 min) (veh/h)|Ramp Flow Rates by Analysis Period (veh/h)<br>ONR1 ONR2a ONR3 OFR1 OFR2 OFR3|Exiting<br>Flow<br>Rate<br>(veh/h)|
|---|---|---|
|1<br>5,001<br>2<br>5,500<br>3<br>5,800<br>4<br>5,200<br>5<br>4,201|500<br>599 (56)<br>500<br>300<br>400<br>300<br>599<br>799 (111)<br>599<br>400<br>400<br>300<br>699<br>899 (167)<br>699<br>300<br>400<br>500<br>400<br>400 (89)<br>500<br>300<br>400<br>300<br>200<br>300 (56)<br>300<br>300<br>200<br>200|5,600<br>6,399<br>6,899<br>5,500<br>4,301|


Note: _a_ Numbers in parentheses indicate ONR-2 to OFR-2 demand flow rates in Weaving
Segment 6.


The values in Exhibit 25-53 represent the adjusted demand flows on the
facility as determined from field observations or demand projections. The
actual volume served in each segment will be determined during the
application of the methodology and is expected to be less downstream of a
congested segment. The demand flows are given for the extended time–space
domain, consistent with the methodology presented in Chapter 10. Peaking
occurs in the third 15-min period. Because inputs are in the form of 15-min
observations, no peak hour factor adjustment is necessary. Additional
geometric and traffic-related inputs are as specified in Exhibit 25-44 and the
Facts section of the problem statement.


**Step A-4: Balance Demands**

The traffic flows in Exhibit 25-53 have already been given in the form of
actual demands and no balancing is necessary.


**Step A-5: Identify Global Parameters**

Global inputs are jam density and queue discharge capacity drop. Values
for both parameters are given in the Facts section of the problem statement.


**Step A-6: Code Base Facility**

In this step, all input data for the subject are coded in the computational
engine. Note that this facility can be coded by increasing entry demand
across the facility by 11% relative to the Example Problem 1 demands.


**Step A-7: Compute Segment Capacities**

Because no changes to segment geometry were made, the segment
capacities for basic and ramp segments are consistent with Example Problem
1. Capacities for weaving segments are a function of weaving flow patterns,
and the increased demand flows resulted in slight changes as shown in
Exhibit 25-54.


**Exhibit 25-54: Example Problem 2: Segment Capacities**







|Analysis<br>Period|Capacities (veh/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|6,748 6,748 6,748 6,748 6,748<br>8,273<br>6,748 6,748 6,748 6,748 6,748<br>8,281<br>8,323<br>8,403<br>8,463|


**Step A-8: Calibrate with Adjustment Factors**

This step allows the analyst to adjust demands, capacities, and FFSs for
the purpose of calibration. There is no adjustment needed for the subject
capacity according to the problem statement.


**Step A-9: Adjust Managed Lane Cross Weave**

This step is only required for facilities with managed lanes. The subject
facility does not have a managed lane.


**Step A-10: Compute Demand-to-Capacity Ratios**

The demand-to-capacity ratios in Exhibit 25-55 are calculated from the
demand flows in Exhibit 25-53 and the segment capacities in Exhibit 25-54.


**Exhibit 25-55: Example Problem 2: Segment Demand-to-Capacity Ratios**








|Analysis Period|Demand-to-Capacity Ratios by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2|0.74<br>0.82<br>0.82<br>0.82<br>0.77<br>0.70<br>0.80<br>0.87<br>0.87<br>0.87<br>0.83<br>0.82<br>0.90<br>0.90<br>0.90<br>0.84<br>0.78<br>0.90<br>0.99<br>0.99<br>0.99<br>0.95|


The computed _vd/c_ matrix in Exhibit 25-55 shows Segments 8–11 have
_vd/c_ ratios greater than 1.0 (bold values). Consequently, the facility is
categorized as _oversaturated_, and the analysis proceeds with computing the
oversaturated service measures in Step A-12. It is expected that queuing will
occur on the facility upstream of the congested segments and that the volume
served in each segment downstream of the congested segments will be less
than its demand. This residual demand will be served in later time intervals,
provided the upstream demand drops and queues are allowed to clear.


**Step A-12: Compute Oversaturated Segment Service Measures**

Computations for oversaturation apply to any segment with a _vd/c_ ratio
greater than 1.0 as well as any segments upstream of those segments that
experience queuing as a result of the bottleneck. All remaining segments are
analyzed by using the individual segment methodologies of Chapters 12, 13,
and 14, as applicable, with the caveat that volumes served may differ from
demand flows.

Similar to Example Problem 1, in Example Problem 2 the methodology
calculates performance measures for each segment and each analysis period,
starting with the first segment in Analysis Period 1. The computations are
repeated for all segments for Analysis Periods 1 and 2 without encountering
a segment with _vd/c_ - 1.0. Once the methodology enters Analysis Period 3
and Segment 8, the oversaturated computational module is invoked.

At the first active bottleneck, the _va/c_ ratio for Segment 8 will be exactly
1.0 and the segment will process traffic at its capacity. Consequently, demand
for all downstream segments will be metered by that bottleneck. The
unsatisfied demand is stored in upstream segments, which causes queuing in
Segment 7 and perhaps segments further upstream depending on the level of
excess demand. The rate of growth of the vehicle queue (wave speed) is
estimated from shock wave theory. The performance measures (speed and
density) of any segment with queuing are recomputed, and the newly
calculated values override the results from the segment-specific procedures.


Any unsatisfied demand is served in later analysis periods. As a result,
volumes served in later analysis periods may be higher than the period
demand flows. The resulting matrix of volumes served for Example Problem
2 is shown in Exhibit 25-56.


**Exhibit 25-56: Example Problem 2: Volume-Served Matrix**







|Analysis<br>Period|Volumes Served (veh/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|5,001 5,500 5,500 5,500 5,200 5,800 5,400 5,900 5,900 5,900 5,600<br>5,500 6,099 6,099 6,099 5,700 6,499 6,099 6,699 6,699 6,699 6,399<br>5,800 6,499 6,499 6,499 5,831 6,281 5,584 6,284 6,284 6,284 5,859<br>5,200 5,600 5,600 5,600 5,668 6,311 5,776 6,276 6,276 6,276 5,934<br>4,201 4,401 4,401 4,401 4,102 4,608 4,840 5,140 5,140 5,140 4,912|


As a result of the bottleneck activation in Segment 8 in Analysis Period
3, queues form in upstream Segments 7, 6, and 5. The queuing is associated
with reduced speeds and increased densities in those segments. The results in
this chapter were obtained from the computational engine. The resulting
performance measures computed for each segment and time interval are
speed (Exhibit 25-57), density (Exhibit 25-58), and LOS (Exhibit 25-59).


**Exhibit 25-57: Example Problem 2: Speed Matrix**







|Analysis Period|Speed (mi/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|59.8<br>53.2<br>58.6<br>55.9<br>59.5<br>46.8<br>59.0<br>52.5<br>52.5<br>55.7<br>58.3<br>58.6<br>52.1<br>55.8<br>55.5<br>57.9<br>45.4<br>55.8<br>50.6<br>50.6<br>51.5<br>53.9<br>57.4<br>51.1<br>53.1<br>53.1<br>45.3<br>24.2<br>28.1<br>51.6<br>51.6<br>54.7<br>57.1<br>47.2<br>47.5<br>51.5<br>48.3<br>56.5<br>24.7<br>29.6<br>51.7<br>51.7<br>54.7<br>56.8<br>60.0<br>54.5<br>59.7<br>56.2<br>60.0<br>51.4<br>50.9<br>53.7<br>53.7<br>56.1<br>59.9|


**Exhibit 25-58: Example Problem 2: Density Matrix**








|Analysis Period|Density (veh/mi/ln) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4|27.9<br>34.5<br>31.3<br>32.8<br>29.2<br>31.0<br>30.5<br>37.4<br>37.4<br>35.3<br>32.0<br>31.3<br>39.0<br>36.4<br>36.7<br>32.8<br>35.8<br>36.4<br>44.2<br>44.2<br>43.3<br>39.6<br>33.7<br>42.4<br>40.8<br>40.8<br>42.9<br>64.8<br>66.4<br>40.6<br>40.6<br>38.3<br>34.2<br>36.7<br>39.3<br>36.3<br>38.6<br>33.4<br>63.9<br>65.1<br>40.4<br>40.4<br>38.2<br>34.8|


**Exhibit 25-59: Example Problem 2: Expanded LOS Matrix**













|Analysis Period|Density-Based LOS by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|D<br>D<br>D<br>D<br>D<br>D<br>D<br>D<br>E<br>D<br>D<br>D<br>D<br>E<br>D<br>D<br>E<br>E<br>E<br>E<br>D<br>E<br>D<br>D<br>E<br>D<br>E<br>F<br>F<br>D<br>E<br>D<br>D<br>E<br>E<br>E<br>E<br>D<br>F<br>F<br>D<br>E<br>D<br>E<br>C<br>C<br>C<br>C<br>C<br>C<br>D<br>C<br>D<br>C<br>D|
|**Analysis Period**|**Demand-Based LOS by Segment**<br>**1**<br>**2**<br>**3**<br>**4**<br>**5**<br>**6**<br>**7**<br>**8**<br>**9**<br>**10**<br>**11**|
|1<br>2<br>3<br>4<br>5|**F**<br>**F**<br>**F**<br>**F**|


The LOS table for oversaturated facilities (Exhibit 25-59) distinguishes
between the conventional density-based LOS and a segment demand-based
LOS. The density-based stratification strictly depends on the prevailing
average density on each segment. Segments downstream of the bottleneck,
whose capacities are greater than or equal to the bottleneck capacity, operate
at LOS E (or better), even though their _vd/c_ ratios are greater than 1.0. The
demand-based LOS identifies those segments with demand-to-capacity ratios
exceeding 1.0 as if they had been evaluated in isolation (i.e., using the
methodologies of Chapters 12, 13, and 14). By contrasting the two parts of
the LOS table, the analyst can develop an understanding of the metering effect
of the bottleneck.


**Step A-13: Apply Managed Lane Adjacent Friction Factor**

This step is only required for facilities with managed lanes.


**Step A-14: Compute Lane Group Performance**

This step is only required for facilities with managed lanes.


**Step A-15: Compute Freeway Facility Service Performance**
**Measures by Time Interval**

In the final analysis step, facilitywide performance measures are
calculated for each time interval (Exhibit 25-60), consistent with Example
Problem 1. Because the computations have already been shown, only
summary results are shown here.


**Exhibit 25-60: Example Problem 2: Facility Performance Measure Summary**







|Analysis Period|Performance Measure<br>Space Mean Speed (mi/h) Average Density (veh/mi/ln)|LOS|
|---|---|---|
|1<br>2<br>3<br>4<br>5|56.8<br>31.0<br>54.4<br>36.2<br>42.5<br>45.6<br>42.5<br>43.8<br>56.4<br>26.2|D<br>E<br>F<br>E<br>D|
|**Total**|**50.5**<br>**35.6**|**—**|


**Step A-16: Aggregate to Section Level and Validate Against Field**
**Data**

This step validates the analysis and is performed only when field data
are available.


**Step A-17: Estimate LOS and Report Performance Measures for**
**Lane Groups and Facility**

The LOS for each time interval is determined directly from the average
density for each time interval. The facility operates at LOS F in Analysis
Period 3 because one or more individual segments have demand-to-capacity
ratios = 1.0, even though the average facility density is below the LOS F
threshold.


**EXAMPLE PROBLEM 3: CAPACITY IMPROVEMENTS TO AN**
**OVERSATURATED FACILITY**


**The Facility**


In this example, portions of the congested facility in Example Problem 2
are being improved in an attempt to alleviate the congestion resulting from
the Segment 8 bottleneck. Exhibit 25-61 shows the upgraded facility
geometry.


**Exhibit 25-61:Example Problem 3: Freeway Facility**


The modified geometry of the 6-mi directional freeway facility is
reflected in Exhibit 25-62.


**Exhibit 25-62: Example Problem 3: Geometry of Directional Freeway Facility**


**Segment No.** **1** **2** **3** **4** **5** **6** **7** **8** **9** **10** **11**

B or
Segment type B ONR B OFR B B ONR R OFR B
W

Segment
5,280 1,500 2,280 1,500 5,280 2,640 5,280 1,140 360 1,140 5,280
length (ft)

No. of lanes 3 3 3 3 3 4 **4** **4** **4** **4** **4**


Notes: B = basic freeway segment; W = weaving segment; ONR = on-ramp (merge)
segment; OFR = off-ramp (diverge) segment; R = overlapping ramp segment.
Bold type indicates geometry changes from Example Problems 1 and 2.


The facility improvements consisted of adding a lane to Segments 7–11 to
give the facility a continuous four-lane cross section starting in Segment 6.
The active bottleneck in Example Problem 2 was in Segment 8, but prior
analysis showed that other segments (Segments 9–11) showed similar
demand-to-capacity ratios greater than 1.0. Consequently, any capacity
improvements that are limited to Segment 8 would have merely moved the
spatial location of the bottleneck farther downstream rather than improving
the overall facility. Segments 9–11 may also be referred to as “hidden” or
“inactive” bottlenecks, because their predicted congestion is mitigated by the
upstream metering of traffic.


**The Facts**

In addition to the information contained in Exhibit 25-61 and Exhibit 2562, the following characteristics of the freeway facility are known:

SUTs and buses = 1.25% (all movements);

Mainline TTs = 1.00% (all movements);

Driver population = regular commuters;

_FFS_ = 60 mi/h (all mainline segments);

Ramp _FFS_ = 40 mi/h (all ramps);

Acceleration lane = 500 ft (all ramps);
length

Deceleration lane = 500 ft (all ramps);
length

_D_ jam = 190 pc/mi/ln;

_cIFL_ = 2,300 pc/h/ln (for _FFS_ = 60 mi/h);

_Ls_ = 1,640 ft (for Weaving Segment 6);

_TRD_ = 1.0 ramp/mi;

Terrain = level;

Analysis duration = 75 min (divided into five 15-min intervals);
and

Demand adjustment = +11% (all segments and all time intervals).


A queue discharge capacity drop of 7% is assumed.


**Comments**

The traffic demand flow inputs are identical to those in Example Problem
2, which reflected an 11% increase in traffic applied to all segments and all
analysis periods relative to Example Problem 1. In an attempt to solve the
congestion effect found in the earlier example, the facility was widened in


Segments 7 through 11. This change directly affects the capacities of those
segments.

In a more subtle way, the proposed modifications also change some of the
defining parameters of Weaving Segment 6. With the added continuous lane
downstream of the segment, the required number of lane changes from the
ramp to the freeway is reduced from one to zero, following the guidelines in
Chapter 13. These changes need to be considered when the undersaturated
performance of that segment is evaluated. The weaving segment’s capacity is
unchanged relative to Example Problem 2 because, even with the proposed
improvements, the number of weaving lanes remains two.


**Step A-1: Define Study Scope**

Similar to the previous example, the number of analysis periods is five
and the facility has 11 segments. The analysis does not include any
methodological extensions (i.e., managed lanes, reliability, ATDM, work
zones).


**Step A-2: Divide Facility into Sections and Segments**

The segmentation of the subject facility is the same as in Example
Problems 1 and 2 and is given in Exhibit 25-61. Therefore, the segmentation
process is not repeated.


**Step A-3: Input Data**

Traffic demand inputs for all 11 segments and five analysis periods are
identical to those in Example Problem 2, as shown in Exhibit 25-53. The
values represent the adjusted demand flows on the facility as determined
from field observations or other sources. The actual volume served in each
segment will be determined by using the methodologies and is expected to be
less downstream of a congested segment. Additional geometric and trafficrelated inputs are as specified in Exhibit 25-62 and the Facts section of the
problem statement.


**Step A-4: Balance Demands**


The traffic flows in Exhibit 25-53 have already been given in the form of
actual demands and no balancing is necessary.


**Step A-5: Identify Global Parameters**

Global inputs are jam density and queue discharge capacity drop. Values
for both parameters are given in the Facts section of the problem statement.


**Step A-6: Code Base Facility**

In this step, all input data for the subject are coded in the computational
engine.


**Step A-7: Compute Segment Capacities**

Segment capacities are determined by using the methodologies of Chapter
12 for basic freeway segments, Chapter 13 for weaving segments, and
Chapter 14 for merge and diverge segments. The resulting capacities are
shown in Exhibit 25-63. Because the capacity of a weaving segment depends
on traffic patterns, it varies by analysis period. The remaining capacities are
constant for all five analysis periods. The capacities for Segments 1–5 and
Segments 7–11 are the same because the segments have the same basic cross
section.


**Exhibit 25-63: Example Problem 3: Segment Capacities**







|Analysis<br>Period|Capacities (veh/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|6,748 6,748 6,748 6,748 6,748<br>8,273<br>8,998 8,998 8,998 8,998 8,998<br>8,281<br>8,323<br>8,403<br>8,463|


**Step A-8: Calibrate with Adjustment Factors**

This step allows the user to adjust demands, capacities, and FFSs for the
purpose of calibration. There is no adjustment needed for the subject
capacity according to the problem statement.


**Step A-9: Adjust Managed Lane Cross Weave**

This step is only required for facilities with managed lanes. The subject
facility does not have a managed lane.


**Step A-10: Compute Demand-to-Capacity Ratios**

The demand-to-capacity ratios in Exhibit 25-64 are calculated from the
demand flows in Exhibit 25-53 and segment capacities in Exhibit 25-63.


**Exhibit 25-64: Example Problem 3: Segment Demand-to-Capacity Ratios**







|Analysis Period|Demand-to-Capacity Ratio by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|0.74<br>0.82<br>0.82<br>0.82<br>0.77<br>0.70<br>0.60<br>0.66<br>0.66<br>0.66<br>0.62<br>0.82<br>0.90<br>0.90<br>0.90<br>0.84<br>0.78<br>0.68<br>0.74<br>0.74<br>0.74<br>0.71<br>0.86<br>0.96<br>0.96<br>0.96<br>0.92<br>0.85<br>0.74<br>0.82<br>0.82<br>0.82<br>0.77<br>0.77<br>0.83<br>0.83<br>0.83<br>0.79<br>0.68<br>0.59<br>0.64<br>0.64<br>0.64<br>0.61<br>0.62<br>0.65<br>0.65<br>0.65<br>0.61<br>0.52<br>0.47<br>0.50<br>0.50<br>0.50<br>0.48|


The demand-to-capacity ratio matrix for Example Problem 3 (Exhibit 2564) shows the capacity improvements successfully reduced all the previously
congested segments to _vd/c_ < 1.0. Therefore, it is expected that the facility
will operate as _globally undersaturated_ and that all segment performance
measures can be directly computed by using the methodologies in Chapters
12, 13, and 14.


**Step A-11: Compute Undersaturated Segment Service Measures**

Because the facility is globally undersaturated, the methodology proceeds
to calculate service measures for each segment and each analysis period,
starting with the first segment in Analysis Period 1. The computational
details for each segment type are exactly as described in Chapters 12, 13,
and 14. The basic performance service measures computed for each segment
and each time interval include segment speed (Exhibit 25-65), density
(Exhibit 25-66), and LOS (Exhibit 25-67).


**Exhibit 25-65: Example Problem 3: Speed Matrix**


|Analysis Period|Speed (mi/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|59.8<br>53.2<br>58.6<br>55.9<br>59.5<br>50.5<br>60.0<br>54.9<br>54.9<br>58.1<br>60.0<br>58.6<br>52.1<br>55.8<br>55.5<br>57.9<br>50.1<br>60.0<br>54.3<br>54.3<br>57.7<br>60.0<br>57.4<br>51.1<br>53.1<br>53.1<br>55.2<br>49.7<br>59.8<br>53.6<br>53.6<br>57.2<br>59.5<br>59.5<br>53.0<br>58.3<br>55.8<br>59.2<br>50.8<br>60.0<br>55.0<br>55.0<br>58.1<br>60.0<br>60.0<br>54.5<br>59.7<br>56.2<br>60.0<br>53.4<br>60.0<br>55.9<br>55.9<br>58.8<br>60.0|


**Exhibit 25-66: Example Problem 3: Density Matrix**







|Analysis Period|Density (veh/mi/ln) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|27.9<br>34.5<br>31.3<br>32.8<br>29.2<br>28.7<br>22.5<br>26.8<br>26.8<br>25.4<br>23.3<br>31.3<br>39.0<br>36.4<br>36.7<br>32.8<br>32.5<br>25.4<br>30.9<br>30.9<br>29.0<br>26.7<br>33.7<br>42.4<br>40.8<br>40.8<br>37.4<br>35.7<br>28.0<br>34.5<br>34.5<br>32.4<br>29.0<br>29.2<br>35.2<br>32.0<br>33.4<br>29.8<br>28.1<br>22.1<br>26.4<br>26.4<br>24.9<br>22.9<br>23.3<br>26.9<br>24.5<br>26.1<br>22.8<br>20.6<br>17.5<br>20.1<br>20.1<br>19.1<br>17.9|


**Exhibit 25-67: Example Problem 3: LOS Matrix**







|Analysis Period|LOS by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|D<br>D<br>D<br>D<br>D<br>D<br>C<br>C<br>D<br>C<br>C<br>D<br>D<br>E<br>D<br>D<br>D<br>C<br>C<br>D<br>C<br>D<br>D<br>D<br>E<br>D<br>E<br>E<br>D<br>D<br>D<br>D<br>D<br>D<br>D<br>D<br>D<br>D<br>D<br>C<br>C<br>D<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>B<br>B<br>C<br>B<br>B|


**Step A-13: Apply Managed Lane Adjacent Friction Factor**

This step is only required for facilities with managed lanes.


**Step A-14: Compute Lane Group Performance**

This step is only required for facilities with managed lanes.


**Step A-15: Compute Freeway Facility Service Performance**
**Measures by Time Interval**


In this analysis step, facilitywide performance measures are calculated
for each analysis period (Exhibit 25-68), consistent with Example Problem
2. Because the computations have already been shown, only summary results
are shown here. The improvement restored the facility LOS to the values
experienced in the original pregrowth scenario, as shown in Exhibit 25-68.


**Exhibit 25-68: Example Problem 3: Facility Performance Measure Summary**







|Analysis Period|Performance Measure<br>Space Mean Speed (mi/h) Average Density (veh/mi/ln)|LOS|
|---|---|---|
|1<br>2<br>3<br>4<br>5|57.9<br>26.8<br>57.1<br>30.3<br>55.9<br>33.5<br>57.8<br>26.9<br>58.6<br>20.8|D<br>D<br>D<br>D<br>C|
|**Total**|**57.5**<br>**27.7**|**—**|


**Step A-16: Aggregate to Section Level and Validate Against Field**
**Data**

This step validates the analysis and is performed only when field data
are available.


**Step A-17: Estimate LOS and Report Performance Measures for**
**Lane Groups and Facility**

The LOS for each time interval is determined directly from the average
density for each time interval. The improvement restored the facility LOS to
the values experienced in the original pregrowth (undersaturated) scenario
shown in Exhibit 25-51.


**EXAMPLE PROBLEM 4: EVALUATION OF AN**
**UNDERSATURATED FACILITY WITH A WORK ZONE**


**The Facility**

In this example, a long-term work zone is placed on the final segment of
Example Problem 1. Exhibit 25-69 shows the change to the facility.


**Exhibit 25-69: Example Problem 4: Freeway Facility**


The modified geometry of the 6-mi directional freeway facility is
reflected in Exhibit 25-70.


**Exhibit 25-70: Example Problem 4: Geometry of Directional Freeway Facility**


**Segment No.** **1** **2** **3** **4** **5** **6** **7** **8** **9** **10** **11**

B or
Segment type B ONR B OFR B B ONR R OFR B
W

Segment
5,280 1,500 2,280 1,500 5,280 2,640 5,280 1,140 360 1,140 5,280
length (ft)

No. of lanes 3 3 3 3 3 4 3 3 3 3 2


Notes: B = basic freeway segment; W = weaving segment; ONR = on-ramp (merge)
segment; OFR = off-ramp (diverge) segment; R = overlapping ramp segment.


**The Facts**

In addition to the information contained in Exhibit 25-69 and Exhibit 2570, the following characteristics of the freeway facility are known:

SUTs and buses = 1.25% (all movements);

Mainline TTs = 1.00% (all movements);

Driver population = regular commuters;

_FFS_ = 60 mi/h (all mainline segments);

Ramp _FFS_ = 40 mi/h (all ramps);

Acceleration lane length = 500 ft (all ramps);

Deceleration lane length = 500 ft (all ramps);

_D_ jam = 190 pc/mi/ln;

_cIFL_ = 2,300 pc/h/ln (for _FFS_ = 60 mi/h);


_Ls_ = 1,640 ft (for Weaving Segment 6);

_TRD_ = 1.0 ramp/mi;

Terrain = level; and

Analysis duration = 75 min (divided into five 15-min intervals).


A queue discharge capacity drop of 7% is assumed for non–work zone
conditions.


**Comments**

The traffic demand flow inputs are identical to those in Example Problem
1. The work zone has a single lane closure (in Segment 11), plastic drum
barriers, and a lateral distance of 0 ft in an urban area. Daytime performance
is of interest throughout the analysis.


**Step A-1: Define Study Scope**

Similar to the previous examples, there are five analysis periods and the
facility has 11 segments. The work zone extension to the methodology will be
included as part of the analysis.


**Step A-2: Divide Facility into Sections and Segments**

The segmentation of the subject facility is given in Exhibit 25-69.
Therefore, there is no need to go through the segmentation process.


**Step A-3: Input Data**

Traffic demand inputs for all 11 segments and five analysis periods are
identical to those in Example Problem 1, as shown in Exhibit 25-45. The
values represent the adjusted demand flows on the facility as determined
from field observations or other sources. Additional geometric and trafficrelated inputs are as specified in Exhibit 25-70 and the Facts section of the
problem statement.


**Step A-4: Balance Demands**


The traffic flows in Exhibit 25-45 have already been given in the form of
actual demands and no balancing is necessary.


**Step A-5: Identify Global Parameters**

Global inputs are jam density and queue discharge capacity drop. Values
for both parameters are given in the Facts section of the problem statement.


**Step A-6: Code Base Facility**

In this step, all input data for the subject facility are coded in the
computational engine.


**Step A-7: Compute Segment Capacities**

The resulting capacities are shown in Exhibit 25-71. Because the
capacity of a weaving segment depends on traffic patterns, it varies by
analysis period. The remaining capacities are constant for all five analysis
periods. The capacities for Segments 1–5 and for Segments 7–10 are the
same because the segments have the same basic cross section. The lane
closure on Segment 11 reduces its base capacity by 33%. The impacts of
work zone presence on further capacity reduction are assessed in the next
step.


**Exhibit 25-71: Example Problem 4: Segment Capacities**







|Analysis<br>Period|Capacities (veh/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|6,748 6,748 6,748 6,748 6,748<br>8,273<br>6,748 6,748 6,748 6,748 4,499<br>8,281<br>8,323<br>8,403<br>8,463|


**Step A-8: Calibrate with Adjustment Factors**

To calculate the CAF for the work zone (Segment 11), the queue
discharge and prebreakdown capacities are required. As a result of the work
zone, Segment 11 has two open lanes and one closed lane. Therefore, from


Exhibit 10-15, its lane closure severity index _LCSI_ value is equal to 0.75.
Equation 10-8 gives the segment’s queue discharge capacity as follows:


Using Equation 10-9 and assuming a 13.1% queue discharge capacity
drop in work zone conditions, prebreakdown capacity is calculated as
follows:


Then, from Equation 10-11, the work zone CAF is equal to


Using a similar approach, the work zone SAF can be found as follows
from Equation 10-10 and Equation 10-12.


These values will be used to update the capacity and FFS of Segment 11
in all analysis periods. In addition, the number of lanes in the segment will
be reduced to two.


**Step A-9: Adjust Managed Lane Cross Weave**

This step is only required for facilities with managed lanes. The subject
facility does not have a managed lane.


**Step A-10: Compute Demand-to-Capacity Ratios**

The demand-to-capacity ratios shown in Exhibit 25-72 are calculated
from the demand flows in Exhibit 25-45 and segment capacities in Exhibit
25-71.


**Exhibit 25-72: Example Problem 4: Segment Demand-to-Capacity Ratios**


|Analysis Period|Demand-to-Capacity Ratio by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3|0.67<br>0.73<br>0.73<br>0.73<br>0.69<br>0.63<br>0.72<br>0.79<br>0.79<br>0.79<br>1.26<br>0.73<br>0.81<br>0.81<br>0.81<br>0.76<br>0.71<br>0.81<br>0.89<br>0.89<br>0.89<br>1.44<br>0.77<br>0.87<br>0.87<br>0.87<br>0.83<br>0.77<br>0.89<br>0.99<br>0.99<br>0.99<br>1.56|


The demand-to-capacity ratio matrix for Example Problem 4 (Exhibit 2572) shows the presence of the work zone significantly increases the demandto-capacity ratio on Segment 11. Queues are very likely to start to grow and
spill back to upstream segments, and the facility is expected to operate in
oversaturated conditions.


**Step A-12: Compute Oversaturated Segment Service Measures**

The computations for oversaturation apply to any segment with a _vd/c_
ratio greater than 1.0, as well as any segments upstream of those segments
that experience queuing as a result of the bottleneck. All remaining segments
are analyzed by using the individual segment methodologies of Chapters 12,
13, and 14, as applicable, with the caveat that the volumes served may differ
from the demand flows.

Similar to Example Problem 1, in Example Problem 4, the methodology
calculates performance measures for each segment and each analysis period,
starting with the first segment in Analysis Period 1. The computations are
repeated for the first 10 segments for Analysis Period 1 without encountering
a segment with _vd/c_ - 1.0. Once the methodology enters Segment 11 in
Analysis Period 1, the oversaturated computational module is invoked.

The _va/c_ ratio for Segment 11, which has the first active bottleneck, will
be more than 1.0 and the segment will process traffic at its capacity.
Consequently, demand for all downstream segments will be metered by that
bottleneck. The unsatisfied demand is stored in upstream segments, which
causes queuing in Segment 10 and perhaps additional upstream segments,
depending on the level of excess demand. The rate of growth of the vehicle
queue (wave speed) is estimated from shock wave theory. The performance
measures (speed and density) of any segment with queuing are recomputed,
and the newly calculated values override the results from the segmentspecific procedures.

Any unsatisfied demand is served in later analysis periods. As a result,
volumes served in later analysis periods may be higher than the period


demand flows. The resulting matrix of volumes served for Example Problem
4 is shown in Exhibit 25-73.


**Exhibit 25-73: Example Problem 4: Volume-Served Matrix**







|Analysis<br>Period|Volumes Served (veh/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|4,505 4,955 4,955 4,955 4,685 5,225 3,924 4,185 4,126 3,929 3,719<br>4,955 5,495 5,495 5,446 3,947 3,701 3,325 3,878 3,882 3,895 3,714<br>3,275 3,476 3,094 3,031 2,912 3,391 3,250 3,899 3,905 3,929 3,714<br>2,831 3,398 3,474 3,416 3,424 3,914 3,597 4,014 4,004 3,965 3,714<br>3,589 3,991 4,096 3,957 3,452 3,912 3,675 3,923 3,916 3,897 3,714|


As a result of the bottleneck activation (due to the work zone’s presence)
in Segment 11 in Analysis Period 1, queues form in upstream Segments 10, 9,
8, 7, and 6. The queuing is associated with reduced speeds and increased
densities in those segments. These and subsequent results were obtained from
the computational engine. The resulting performance measures computed for
each segment and time interval are speed (Exhibit 25-74), density (Exhibit
25-75), and LOS (Exhibit 25-76). Similar trends are observed in the
following time intervals, with queueing reaching the beginning of the facility.


**Exhibit 25-74: Example Problem 4: Speed Matrix**







|Analysis Period|Speed (mi/h) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|60.0<br>53.9<br>59.7<br>56.1<br>60.0<br>48.0<br>24.2<br>15.9<br>13.0<br>13.0<br>50.4<br>59.9<br>53.2<br>54.5<br>52.3<br>22.2<br>8.9<br>9.4<br>12.3<br>12.2<br>12.2<br>50.5<br>12.9<br>12.8<br>13.1<br>9.7<br>8.0<br>6.5<br>9.1<br>12.4<br>12.4<br>12.4<br>50.5<br>5.9<br>11.0<br>12.9<br>12.8<br>11.5<br>8.3<br>11.0<br>13.1<br>12.7<br>12.7<br>50.5<br>11.0<br>16.4<br>18.6<br>16.4<br>12.3<br>8.3<br>11.2<br>12.5<br>12.3<br>12.3<br>50.5|


**Exhibit 25-75: Example Problem 4: Density Matrix**








|Analysis<br>Period|Density (veh/mi/ln) by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4|25.0<br>30.6<br>27.6<br>29.4<br>26.0<br>27.2<br>54.1<br>87.5<br>100.6 100.6 36.9<br>27.6<br>34.5<br>33.6<br>34.7<br>59.1<br>104.2 117.8 105.5 106.2 106.2 36.8<br>84.6<br>90.6<br>78.7 104.6 121.4 130.1 119.1 104.4 105.4 105.4 36.8<br>159.3 103.4 89.8<br>88.7<br>99.4<br>117.3 109.0 102.5 104.2 104.2 36.8|


**Exhibit 25-76: Example Problem 4: LOS Matrix**







|Analysis Period|LOS by Segment<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|C<br>C<br>D<br>C<br>D<br>C<br>F<br>F<br>F<br>F<br>E<br>D<br>D<br>D<br>D<br>F<br>F<br>F<br>F<br>F<br>F<br>E<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>E<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>E<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>F<br>E|


**Step A-13: Apply Managed Lane Adjacent Friction Factor**

This step is only required for facilities with managed lanes.


**Step A-14: Compute Lane Group Performance**

This step is only required for facilities with managed lanes.


**Step A-15: Compute Freeway Facility Service Performance**
**Measures by Time Interval**

In the final analysis step, facilitywide performance measures are
calculated for each analysis period (Exhibit 25-77). Because the
computations have already been demonstrated in previous example
problems, only summary results are shown. The work zone presence created
significant congestion on the subject facility.


**Exhibit 25-77: Example Problem 4: Facility Performance Measure Summary**








|Analysis Period|Performance Measure<br>Space Mean Speed (mi/h) Average Density (veh/mi/ln)|LOS|
|---|---|---|
|1<br>2<br>3<br>4<br>5|39.2<br>38.4<br>21.8<br>66.1<br>11.5<br>99.1<br>11.3<br>105.5<br>13.7<br>93.4|F<br>F<br>F<br>F<br>F|
|Total|19.5<br>80.5|—|


**Step A-16: Aggregate to Section Level and Validate Against Field**
**Data**

This step validates the analysis and is performed only when field data
are available.


**Step A-17: Estimate LOS and Report Performance Measures for**
**Lane Groups and Facility**

The LOS for each time interval is determined directly from the average
density for each time interval. Work zone presence eroded the facility LOS to
F in all time intervals.


**EXAMPLE PROBLEM 5: EVALUATION OF AN OVERSATURATED**
**FACILITY WITH A MANAGED LANE**


**The Facility**

In this example, a managed lane will be added to the freeway facility
described in Example Problem 2. Exhibit 25-78 shows the new facility
geometry.


**Exhibit 25-78: Example Problem 5: Freeway Facility**


Details of the modified geometry of the 6-mi directional freeway facility
are provided in Exhibit 25-79.


**Exhibit 25-79: Example Problem 5: Geometry of Directional Freeway Facility**


**Segment No.** **1** **2** **3** **4** **5** **6** **7** **8** **9** **10** **11**

B or
Segment type B ONR B OFR B B ONR R OFR B
W

Segment 5,280 1,500 2,280 1,500 5,280 2,640 5,280 1,140 360 1,140 5,280


length (ft)
No. of GP
3 3 3 3 3 4 3 3 3 3 3
lanes

No. of ML 1 1 1 1 1 1 1 1 1 1 1


Notes: B = basic freeway segment; W = weaving segment; ONR = on-ramp (merge)
segment; OFR = off-ramp (diverge) segment; R = overlapping ramp segment; GP =
general purpose; ML = managed lanes.


**The Facts**

In addition to the information contained in Exhibit 25-78 and Exhibit 2579, the following characteristics of the freeway facility are known:

SUTs and buses = 1.25% (all movements);

Mainline TTs = 1.00% (all movements);

Driver population = regular commuters;

_FFS_ = 60 mi/h (all mainline segments);

Ramp _FFS_ = 40 mi/h (all ramps);

Acceleration lane = 500 ft (all ramps);
length

Deceleration lane = 500 ft (all ramps);
length

_D_ jam = 190 pc/mi/ln;

_cIFL_ = 2,300 pc/h/ln (for _FFS_ = 60 mi/h);

_Ls_ = 1,640 ft (for Weaving Segment 6);

_TRD_ = 1.0 ramp/mi;

Terrain = level;

Analysis duration = 75 min (divided into five 15-min intervals);
and

Demand adjustment = +11% (all segments and all time intervals).


A queue discharge capacity drop of 7% is assumed.


**Comments**

The traffic demand flow inputs are identical to those in Example Problem
2. The facility includes a single managed lane separated with marking with
FFS equal to 60 mi/h. The lane is a basic managed lane with no intermediate
access points. It is assumed 20% of entry traffic demand on the mainline will
use the managed lane.


**Step A-1: Define Study Scope**

Similar to the previous examples, there are five analysis periods and the
facility has 11 segments. The managed lane extension to the methodology will
be used for this analysis.


**Step A-2: Divide Facility into Sections and Segments**

The segmentation of the subject facility is given in Exhibit 25-78.
Therefore, the segmentation process is not repeated.


**Step A-3: Input Data**

On- and off-ramp demand flow rates are identical to those of Example
Problem 2, shown in Exhibit 25-53. It is assumed total entry volume is
identical to that of Example Problem 2; however, 20% of total demand is
allocated to the managed lane, and the remaining 80% to the general purpose
lanes, as shown in Exhibit 25-80.


**Exhibit 25-80: Example Problem 5: Demand Inputs on the Mainline**



**Entering Flow Rate**
**on Managed Lane**
**(veh/h)**



**Sum of Entering Flow**
**Rate to the Facility**
**(veh/h)**



**Analysis**
**Period**



**Entering Flow Rate on**
**General Purpose Lanes**
**(veh/h)**



1 4,001 1,000 5,001
2 4,400 1,100 5,500
3 4,640 1,160 5,800
4 4,160 1,040 5,200
5 3,361 840 4,201


**Step A-4: Balance Demands**


The traffic flows in Exhibit 25-53 and Exhibit 25-80 have already been
given in the form of actual demands and no balancing is necessary.


**Step A-5: Identify Global Parameters**

Global inputs are jam density and queue discharge capacity drop. Values
for both parameters are given in the problem statement.


**Step A-6: Code Base Facility**

In this step, all input data for the subject facility are coded in the
computational engine.


**Step A-7: Compute Segment Capacities**

Segment capacities are determined by using the methodologies of Chapter
12 for basic freeway segments (general purpose and managed lanes), Chapter
13 for weaving segments, and Chapter 14 for merge and diverge segments.
The resulting capacities are shown in Exhibit 25-81.


**Exhibit 25-81: Example Problem 5: Segment Capacities**













|Analysis<br>Period|Capacities (veh/h) by Segment for General Purpose Lanes<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|6,748 6,748 6,748 6,748 6,748<br>8,177<br>6,748 6,748 6,748 6,748 6,748<br>8,189<br>8,244<br>8,331<br>8,403|
|**Analysis**<br>**Period**|**Capacities (veh/h) by Segment for Managed Lane**<br>**1**<br>**2**<br>**3**<br>**4**<br>**5**<br>**6**<br>**7**<br>**8**<br>**9**<br>**10**<br>**11**|
|1<br>2<br>3<br>4<br>5|1,614 1,614 1,614 1,614 1,614<br>1,614 1,614 1,614 1,614 1,614<br>1,614|


**Step A-8: Calibrate with Adjustment Factors**


This step allows the analyst to adjust demands, capacities, and FFSs for
the purpose of calibration. According to the problem statement, there is no
adjustment needed for the subject facility’s capacity.


**Step A-9: Adjust Managed Lane Cross Weave**

This facility does not have a cross weave. Therefore, this step is
skipped.


**Step A-10: Compute Demand-to-Capacity Ratios**

The demand-to-capacity ratios shown in Exhibit 25-82 are calculated
from the demand flows in Exhibit 25-53 and Exhibit 25-80 and segment
capacities in Exhibit 25-81.


**Exhibit 25-82: Example Problem 5: Segment Demand-to-Capacity Ratios**













|Analysis Period|Demand-to-Capacity Ratio by Segment (General Purpose Lanes)<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|0.59<br>0.67<br>0.67<br>0.67<br>0.62<br>0.59<br>0.65<br>0.73<br>0.73<br>0.73<br>0.68<br>0.65<br>0.74<br>0.74<br>0.74<br>0.68<br>0.66<br>0.74<br>0.83<br>0.83<br>0.83<br>0.79<br>0.69<br>0.79<br>0.79<br>0.79<br>0.75<br>0.72<br>0.82<br>0.92<br>0.92<br>0.92<br>0.85<br>0.62<br>0.68<br>0.68<br>0.68<br>0.63<br>0.56<br>0.63<br>0.71<br>0.71<br>0.71<br>0.66<br>0.50<br>0.53<br>0.53<br>0.53<br>0.48<br>0.42<br>0.50<br>0.54<br>0.54<br>0.54<br>0.51|
|**Analysis Period**|**Demand-to-Capacity Ratio by Segment (Managed Lane)**<br>**1**<br>**2**<br>**3**<br>**4**<br>**5**<br>**6**<br>**7**<br>**8**<br>**9**<br>**10**<br>**11**|
|1<br>2<br>3<br>4<br>5|0.62<br>0.62<br>0.62<br>0.62<br>0.62<br>0.62<br>0.62<br>0.62<br>0.62<br>0.62<br>0.62<br>0.68<br>0.68<br>0.68<br>0.68<br>0.68<br>0.68<br>0.68<br>0.68<br>0.68<br>0.68<br>0.68<br>0.72<br>0.72<br>0.72<br>0.72<br>0.72<br>0.72<br>0.72<br>0.72<br>0.72<br>0.72<br>0.72<br>0.64<br>0.64<br>0.64<br>0.64<br>0.64<br>0.64<br>0.64<br>0.64<br>0.64<br>0.64<br>0.64<br>0.52<br>0.52<br>0.52<br>0.52<br>0.52<br>0.52<br>0.52<br>0.52<br>0.52<br>0.52<br>0.52|


The demand-to-capacity ratio matrix for Example Problem 5 (Exhibit 2582) shows the addition of the managed lane improves traffic operations on
the general purpose lanes. As such, it is expected the facility will operate in
undersaturated conditions.


**Step A-11: Compute Undersaturated Segment Service Measures**


The computations for oversaturation apply to any segment with a _vd/c_
ratio greater than 1.0 as well as any segments upstream of those segments that
experience queuing as a result of the bottleneck. All remaining segments are
analyzed by using the individual segment methodologies of Chapters 12, 13,
and 14, as applicable, with the caveat that volumes served may differ from
demand flows.

The basic performance service measures computed for each segment and
each time interval include segment speed (Exhibit 25-83), density (Exhibit
25-84), and LOS (Exhibit 25-85).


**Exhibit 25-83: Example Problem 5: Speed Matrix**













|Analysis Period|Speed (mi/h) by Segment (General Purpose Lanes)<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|60.0<br>54.4<br>59.7<br>56.2<br>60.0<br>48.0<br>60.0<br>54.0<br>54.0<br>56.1<br>60.0<br>60.0<br>53.8<br>59.7<br>55.9<br>60.0<br>46.8<br>59.8<br>53.0<br>53.0<br>55.8<br>59.2<br>60.0<br>53.3<br>59.1<br>55.9<br>59.7<br>46.2<br>58.5<br>51.7<br>51.7<br>55.0<br>57.7<br>60.0<br>54.3<br>59.7<br>56.2<br>60.0<br>49.9<br>60.0<br>54.1<br>54.1<br>56.1<br>60.0<br>60.0<br>55.2<br>59.8<br>56.3<br>60.0<br>52.7<br>60.0<br>55.1<br>55.1<br>56.5<br>60.0|
|**Analysis Period**|**Speed (mi/h) by Segment (Managed Lane)**<br>**1**<br>**2**<br>**3**<br>**4**<br>**5**<br>**6**<br>**7**<br>**8**<br>**9**<br>**10**<br>**11**|
|1<br>2<br>3<br>4<br>5|59.3<br>59.3<br>59.3<br>59.3<br>59.3<br>59.3<br>59.3<br>59.3<br>59.3<br>59.3<br>59.3<br>58.9<br>58.9<br>58.9<br>58.9<br>58.9<br>58.9<br>58.9<br>53.5<br>53.5<br>58.1<br>58.9<br>58.6<br>58.6<br>58.6<br>58.6<br>58.6<br>58.6<br>58.6<br>52.1<br>52.1<br>52.1<br>58.6<br>59.2<br>59.2<br>59.2<br>59.2<br>59.2<br>59.2<br>59.2<br>59.2<br>59.2<br>59.2<br>59.2<br>59.7<br>59.7<br>59.7<br>59.7<br>59.7<br>59.7<br>59.7<br>59.7<br>59.7<br>59.7<br>59.7|


**Exhibit 25-84: Example Problem 5: Density Matrix**














|Analysis Period|Density (veh/mi/ln) by Segment (General Purpose Lanes)<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|22.2<br>27.6<br>25.0<br>26.7<br>23.3<br>25.0<br>24.4<br>30.3<br>30.3<br>29.1<br>25.6<br>24.4<br>31.0<br>27.9<br>29.8<br>25.6<br>28.9<br>27.9<br>35.2<br>35.2<br>33.4<br>29.8<br>25.8<br>33.4<br>30.1<br>31.8<br>28.1<br>32.2<br>31.6<br>40.2<br>40.2<br>37.8<br>33.2<br>23.1<br>28.0<br>25.3<br>27.1<br>23.7<br>23.4<br>23.7<br>29.3<br>29.3<br>28.3<br>24.8<br>18.7<br>21.5<br>19.8<br>21.1<br>18.1<br>16.9<br>18.7<br>22.1<br>22.1<br>21.6<br>19.2|
|**Analysis Period**|**Density (veh/mi/ln) by Segment (Managed Lane)**<br>**1**<br>**2**<br>**3**<br>**4**<br>**5**<br>**6**<br>**7**<br>**8**<br>**9**<br>**10**<br>**11**|
|1<br>2|16.9<br>16.9<br>16.9<br>16.9<br>16.9<br>16.9<br>16.9<br>16.9<br>16.9<br>16.9<br>16.9<br>18.7<br>18.7<br>18.7<br>18.7<br>18.7<br>18.7<br>18.7<br>20.6<br>20.6<br>18.7<br>18.7|


**Exhibit 25-85: Example Problem 5: LOS Matrix**













|Analysis Period|LOS by Segment (General Purpose Lanes)<br>1 2 3 4 5 6 7 8 9 10 11|
|---|---|
|1<br>2<br>3<br>4<br>5|C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>D<br>C<br>C<br>C<br>C<br>D<br>C<br>C<br>D<br>D<br>D<br>E<br>D<br>D<br>C<br>D<br>D<br>D<br>D<br>D<br>D<br>D<br>E<br>D<br>D<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>D<br>C<br>C<br>C<br>B<br>C<br>C<br>C<br>B<br>C<br>B<br>C<br>C<br>C|
|**Analysis Period**|**LOS by Segment (Managed Lane)**<br>**1**<br>**2**<br>**3**<br>**4**<br>**5**<br>**6**<br>**7**<br>**8**<br>**9**<br>**10**<br>**11**|
|1<br>2<br>3<br>4<br>5|B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>C<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B<br>B|


**Step A-13: Apply Managed Lane Adjacent Friction Factor**

The subject facility has densities in excess of 35 pc/mi/ln. As a result,
friction effects are applied according to the process described in Chapter 12.
The indicator variable _Ic_ in Equation 12-12 will have a nonzero value for the
segments and analysis periods during which the general purpose lane density
is greater than 35 pc/mi/ln. Consequently, the _S_ 3 term in Equation 12-12 will
reduce the estimated general purpose lane speed as a result of the friction.


**Step A-14: Compute Lane Group Performance**

In this step, performance measures for all the facility’s lane groups are
computed. The subject facility has two lane groups, one for general purpose
lanes and one for the managed lane, as shown in Exhibit 25-86.


**Exhibit 25-86: Example Problem 5: Facility Performance Measure Summary for Lane**
**Groups**


|Period|Performance Measure<br>Space Mean Average Density<br>Speed (mi/h) (veh/mi/ln)|Performance Measure<br>Space Mean Average Density<br>Speed (mi/h) (veh/mi/ln)|
|---|---|---|
|1<br>2<br>3<br>4<br>5|57.7<br>24.9<br>57.3<br>28.1<br>56.5<br>31.0<br>58.0<br>24.6<br>58.5<br>19.1|59.3<br>16.9<br>58.6<br>18.8<br>58.0<br>20.0<br>59.2<br>17.6<br>59.7<br>14.1|


**Step A-15: Compute Freeway Facility Service Performance**
**Measures by Time Interval**

In the final analysis step, facilitywide performance measures are
calculated for each analysis period (Exhibit 25-87). Because the
computations have been demonstrated previously, only summary results are
shown here. The addition of the managed lane reduced traffic congestion on
the subject facility.


**Exhibit 25-87: Example Problem 5: Facility Performance Measure Summary**







|Analysis Period|Performance Measure<br>Space Mean Speed (mi/h) Average Density (veh/mi/ln)|LOS|
|---|---|---|
|1<br>2<br>3<br>4<br>5|58.0<br>23.4<br>57.5<br>26.4<br>56.7<br>29.1<br>58.2<br>23.3<br>58.7<br>18.1|C<br>D<br>D<br>C<br>C|
|Total|57.8<br>24.0|—|


**Step A-16: Aggregate to Section Level and Validate Against Field**
**Data**

This step validates the analysis and is performed only when field data
are available.


**Step A-17: Estimate LOS and Report Performance Measures for**
**Lane Groups and Facility**


The LOS for each time interval is determined directly from the average
density for each time interval. The addition of the managed lane improved
traffic conditions over the entire facility.


**EXAMPLE PROBLEM 6: PLANNING-LEVEL ANALYSIS OF A**
**FREEWAY FACILITY**


**The Facility**

In this example, the planning-level methodology is used to analyze a
freeway facility with geometric characteristics identical to the facility used
in Example Problem 1. Exhibit 25-43 shows the facility geometry. Note that
the planning methodology uses annual average daily traffic (AADT) values to
calculate demand levels at the facility’s entry and exit points based on the
hourly ( _K_ ) and annual growth factors ( _fg_ ). As a result, although the AADTs
have been manipulated in this example to create demand levels close to those
of Example Problem 1, the results will not match precisely. Furthermore,
because the planning-level methodology uses freeway sections rather than
segments and is limited to four analysis periods, a direct comparison is not
possible.


**The Facts**

In addition to the information given in Exhibit 25-43 and Exhibit 25-44,
the following characteristics of the freeway facility are known:

Heavy-vehicle = 0%,
percentage

Driver population = regular commuters on an urban facility,

_FFS_ = 60 mi/h (all mainline segments),

Ramp _FFS_ = 40 mi/h (all ramps),

_D_ jam = 190 pc/mi/ln,

_K-_ factor = 0.09,

Growth factor = 1,

_PHF_ = 0.9,

Terrain = level, and

Analysis duration = 60 min (divided into four 15-min analysis


periods).


**Average Annual Daily Traffic**

The planning-level approach uses directional AADT values to
approximate demand levels on different freeway sections. Exhibit 25-88
depicts AADT values on all entry points (i.e., the first basic freeway section
and all on-ramps) and all exit points (all off-ramps).


**Exhibit 25-88: Example Problem 6: AADT Values for the Facility**







|Entering AADT (veh/day)|Ramp AADT (veh/day)<br>ONR1 ONR2 ONR3 OFR1 OFR2 OFR3|
|---|---|
|55,000|4,500<br>5,400<br>4,500<br>2,700<br>3,600<br>2,700|


**Sections**

The facility and all geometric inputs are identical to Example Problem 1.
Exhibit 25-89 presents the different freeway sections for the facility of
interest.


**Exhibit 25-89: Example Problem 6: Section Definition for the Facility**


Section 1 is a basic section, identical to the HCM segmentation
definition. An on-ramp roadway is located just downstream of Section 1 that
results in changes in the demand level. As a result, a new section needs to be
defined. The demand level on the new section remains fixed up to the first
off-ramp roadway, at which point both the capacity and the demand change.


As a result, Section 2 is defined as a ramp section. After the off-ramp
roadway, the facility demand drops and remains fixed until the next on-ramp
roadway. As a result, Section 3 is defined as a basic freeway section.
Sections on the rest of the freeway facility are defined following a similar
process. The result is that seven distinct sections are defined.


**Step 1: Demand Level Calculations**

The demand level on each section in each analysis period is determined
by using the given AADT values, _PHF_, _K-_ factor, heavy-vehicle factor, and
growth factor.


By following the same approach, the demand levels for all facility entry
and exit points are found. The results are summarized in Exhibit 25-90.


**Exhibit 25-90: Example Problem 6: Demand Flow Rates (pc/h) on the Subject Facility**



**Off-**
**Ramp**
**1**



**On-**
**Ramp**
**2**



**Off-**
**Ramp**
**2**



**On-**
**Ramp**
**3**



**Off-**
**Ramp**
**3**



**Analysis**
**Period** **Entry**



**On-**
**Ramp**
**1**



1 4,950 405 243 486 324 405 243
2 5,500 450 270 540 360 450 270
3 4,950 405 243 486 324 405 243
4 4,400 360 216 432 288 360 216


After calculation of the entry and exit demand flow rates from the AADT
values, the demand level in each section in each analysis period is found.


**Step 2: Section Capacity Calculations and Adjustments**

Equation 25-45 is used to determine the base capacity of each section.
The base capacity of each section is then adjusted by using the appropriate
adjustment factor for a weaving, ramp, merge, or diverge section. For
instance, the capacity of Section 1 (a basic section) is determined as follows:


Because FFS and percentage heavy vehicles are global inputs, the
capacity of each of the facility’s basic freeway sections is equal to 2,300
pc/h/ln. However, for all other sections, this base capacity needs to be
adjusted.

Section 2 is a ramp section. The CAF for a ramp section is 0.9.
Therefore, the capacity of Section 2 is computed as follows:


Section 3 is a basic freeway section; therefore, its capacity remains at
2,300 pc/h/ln. However, Section 4 is a weaving section and its capacity will
need to be adjusted. The CAF for a weaving section is determined by the
volume ratio and section length.

The volume ratio (the ratio of weaving demand to total demand) is
approximated by summing the weaving section’s ramp AADT values and
dividing the result by the total AADT on the weaving section, as follows:


The length of the weaving section is 0.5 mi. As a result, the CAF is
calculated as follows:


Therefore, the capacity of Section 4 is


The capacities of Section 5 (basic), Section 6 (ramp), and Section 7
(basic) are 2,300, 2,070, and 2,300 pc/h/ln, respectively. At this stage,
demand-to-capacity ratios for all sections in all analysis periods can be
determined, as presented in Exhibit 25-91.


**Exhibit 25-91: Example Problem 6: Demand-to-Capacity Ratios by Section and Analysis**
**Period**







|Analysis Period|Demand-to-Capacity Ratios by Section<br>1 2 3 4 5 6 7|
|---|---|
|1<br>2<br>3<br>4|0.72<br>0.86<br>0.74<br>0.65<br>0.76<br>0.91<br>0.79<br>0.80<br>0.96<br>0.82<br>0.72<br>0.85<br>1.02<br>0.88<br>0.72<br>0.86<br>0.74<br>0.65<br>0.76<br>0.93<br>0.80<br>0.64<br>0.77<br>0.66<br>0.58<br>0.68<br>0.81<br>0.70|


As shown in Exhibit 25-91, the demand-to-capacity ratio in the sixth
section in the second analysis period is greater than one. As a result, queue
formation and low space mean speeds are expected on this section. The
demand-to-capacity ratios on the remaining segments are below one across
all analysis periods.


**Step 3: Delay Rate Estimation**

In this step, demand-to-capacity ratios are used to determine delay rates
for all sections of the facility across all analysis periods. FFS on the facility
is 60 mi/h, and all demand-to-capacity ratios are below one. As a result, the
delay rates for each section are found by using Equation 25-47.


For instance, the delay rate for Section 1 in the first analysis period is 0
s/mi, because its demand-to-capacity ratio of 0.717 is less than the 0.72
threshold used in Equation 25-47. Section 2’s demand-to-capacity ratio is
0.86, which is greater than the threshold. Therefore, its delay rate is
calculated as follows:


Delay rates for other sections of the facility are determined in the same
way and are summarized in Exhibit 25-92.


**Exhibit 25-92: Example Problem 6: Delay Rates by Section and Analysis Period**







|Anal|ysis Period|Delay Rate by Section (s/mi)<br>1 2 3 4 5 6 7|
|---|---|---|
||1<br>2<br>3<br>4|0.0<br>2.8<br>0.2<br>0.0<br>0.5<br>5.0<br>0.8<br>1.0<br>7.4<br>1.6<br>0.1<br>2.3<br>11.7<br>3.3<br>0.0<br>2.8<br>0.2<br>0.0<br>0.5<br>5.8<br>1.1<br>0.0<br>0.5<br>0.0<br>0.0<br>0.0<br>1.3<br>0.0|


**Step 4: Average Travel Time, Speed, and Density Calculations**

Delay rates are used to compute travel times and, consequently, speeds.
To determine a section’s travel time, its travel rate is calculated by summing
the section’s travel rate under free-flow conditions and its delay rates for
undersaturated and oversaturated conditions. This calculation is repeated for
each section across all analysis periods. The following equations
demonstrate the calculation for the first two sections during the first analysis
period:


Travel rates for all sections across all analysis periods are shown in
Exhibit 25-93.


**Exhibit 25-93: Example Problem 6: Travel Rates by Section and Analysis Period**







|Analysis Period|Travel Rate by Section (s/mi)<br>1 2 3 4 5 6 7|
|---|---|
|1<br>2<br>3<br>4|60.0<br>62.8<br>60.2<br>60.0<br>60.5<br>65.0<br>60.8<br>61.0<br>67.4<br>61.6<br>60.1<br>62.3<br>71.7<br>63.3<br>60.0<br>62.8<br>60.2<br>60.0<br>60.5<br>65.8<br>61.1<br>60.0<br>60.5<br>60.0<br>60.0<br>60.0<br>61.3<br>60.0|


Each section’s travel time is calculated by multiplying its travel rate by
its length. The results are presented in Exhibit 25-94.


**Exhibit 25-94: Example Problem 6: Average Travel Times by Section and Analysis**
**Period**







|Analysis Period|Travel Time by Section (s)<br>1 2 3 4 5 6 7|
|---|---|
|1<br>2<br>3<br>4|60.0<br>62.8<br>60.2<br>30.0<br>60.5<br>32.5<br>60.8<br>61.0<br>67.4<br>61.6<br>30.0<br>62.3<br>35.8<br>63.3<br>60.0<br>62.8<br>60.2<br>30.0<br>60.5<br>32.9<br>61.1<br>60.0<br>60.5<br>60.0<br>30.0<br>60.0<br>30.7<br>60.0|


Density is determined for each section across all analysis periods by
dividing the section’s demand by its speed (section length divided by travel
time). The results are shown in Exhibit 25-95.


**Exhibit 25-95: Example Problem 6: Density by Section and Analysis Period**







|Analysis Period|Density by Section (pc/mi/ln)<br>1 2 3 4 5 6 7|
|---|---|
|1<br>2<br>3<br>4|27.5<br>31.1<br>28.5<br>23.3<br>29.5<br>34.2<br>30.6<br>31.1<br>37.2<br>32.4<br>25.9<br>33.8<br>41.2<br>35.4<br>27.5<br>31.1<br>28.5<br>23.3<br>29.5<br>35.2<br>31.3<br>24.4<br>26.7<br>25.2<br>20.7<br>26.0<br>28.7<br>26.8|


Finally, the approach provides a high-level summary that includes a
capacity assessment, the aggregated travel time, the space mean speed, the
average facility density, the total queue length, and the facility LOS by
analysis period, as shown in Exhibit 25-96.


**Exhibit 25-96: Example Problem 6: Facility Performance Summary**















|Analysis<br>Period|High-Level<br>Capacity<br>Assessment|Travel<br>Time<br>(min)|Space<br>Mean<br>Speed<br>(mi/h)|Average<br>Facility Density<br>(pc/mi/ln)|Total<br>Queue<br>Length<br>(mi)|LOS|
|---|---|---|---|---|---|---|
|1<br>2<br>3<br>4|Undersaturated<br>Oversaturated<br>Undersaturated<br>Undersaturated|6.1<br>6.4<br>6.1<br>6.0|58.9<br>56.6<br>58.8<br>59.8|29.2<br>33.7<br>29.4<br>25.5|0.0<br>0.8<br>0.0<br>0.0|D<br>F<br>D<br>C|


The average facility travel time in each analysis period is calculated by
summing each section’s travel time and dividing the result by 60 to convert
the units to minutes. Space mean speed in each analysis period is then
calculated by dividing the total facility length by the facility travel time in
each analysis period. The facility density is a length-weighted average of
each section’s density, and the total queue length is the sum of each section’s
queue length. Finally, LOS is calculated based on the urban freeway density
thresholds if the demand-to-capacity ratio is less than 1; otherwise, LOS is
set to F if any section operates at a demand-to-capacity ratio greater than 1.0.

The facility is oversaturated during the second analysis period, with one
of the sections experiencing a demand-to-capacity ratio greater than 1.0. The
method estimates that a 0.8-mi queue will result from an active bottleneck.
With at least one time interval operating at LOS F, it is recommended that a


more detailed operational analysis of this facility be conducted to obtain a
more accurate estimate of congestion patterns.


**EXAMPLE PROBLEM 7: RELIABILITY EVALUATION OF AN**
**EXISTING FREEWAY FACILITY**


**The Facility**

This example problem uses the same 6-mi facility used in Example
Problem 1. The facility consists of 11 segments with the properties indicated
in Exhibit 25-97. Other facility characteristics are identical to those given in
Example Problem 1, except that the study period in this example has been
extended from 75 to 180 min. Exhibit 25-98 shows the facility geometry.


**Exhibit 25-97: Example Problem 7: Freeway Facility**


**Exhibit 25-98: Example Problem 7: Geometry of Directional Freeway Facility**


**Segment No.** **1** **2** **3** **4** **5** **6** **7** **8** **9** **10** **11**

B or
Segment type B ONR B OFR B B ONR R OFR B
W

Segment
5,280 1,500 2,280 1,500 5,280 2,640 5,280 1,140 360 1,140 5,280
length (ft)

No. of lanes 3 3 3 3 3 4 3 3 3 3 3


Notes: B = basic freeway segment; W = weaving segment; ONR = on-ramp (merge)
segment; OFR = off-ramp (diverge) segment; R = overlapping ramp segment.


**Input Data**

This example illustrates the use of defaults and lookup tables to substitute
for desirable but difficult to obtain data. Minimum facility inputs for the
example problem include the following.


_Facility Geometry_

All the geometric information about the facility normally required for an
HCM freeway facility analysis (Chapters 10–14) is also required for a
reliability analysis. These data are supplied as part of the base scenario.


_Study Parameters_

These parameters specify the study period, the reliability reporting
period, and the date represented by the traffic demand data used in the base
scenario.

The study period in this example is from 4 to 7 p.m., which covers the
afternoon and early evening peak hour and shoulder periods. Recurring
congestion is typically present in the study direction of this facility during
that period, which is why it has been selected for reliability analysis. The
reliability reporting period is set as all weekdays in the calendar year. (For
simplicity of presentation in this example, holidays have not been removed
from the reliability reporting period.) The demand data are reflective of
AADT variations across the weekdays and months in a calendar year for the
subject facility.


_Base Demand_

Demand flow rates in vehicles per hour are supplied for each 15-min
analysis period in the base scenario. Care should be taken that demand data
are measured upstream of any queued traffic. If necessary, demand can be
estimated as the sum of departing volume and the change in the queue size at
a recurring bottleneck.

Exhibit 25-99 provides the twelve 15-min demand flow rates required
for the entire 3-h study period.


**Exhibit 25-99: Example Problem 7: Demand Flow Rates (veh/h) by Analysis Period in**
**the Base Data Set**


**Analysis** **Demand Entry Flow**
**Period** **Rate** **ONR1 ONR2 ONR3 OFR1 OFR2 OFR3**

**1** 3,095 270 270 270 180 270 180
**2** 3,595 360 360 360 270 360 270
**3** 4,175 360 450 450 270 360 270


**4** 4,505 450 540 450 270 360 270
**5** 4,955 540 720 540 360 360 270
**6** 5,225 630 810 630 270 360 450
**7** 4,685 360 360 450 270 360 270
**8** 3,785 180 270 270 270 180 180
**9** 3,305 180 270 270 270 180 180
**10** 2,805 180 270 270 270 180 180
**11** 2,455 180 180 180 270 180 180
**12** 2,405 180 180 180 180 180 180


Note: ONR = on-ramp; OFR = off-ramp.


_Incident Data_

Detailed incident logs are not available for this facility, but local data are
available about the facility’s crash rate: 150 crashes per 100 million VMT.
An earlier study conducted by the state in which the facility is located found
that an average of seven incidents occur for every crash.


**Computational Steps**


_Base Data Set Analysis_

The Chapter 10 freeway facilities core methodology is applied to the
base data set to ensure the specified facility boundaries and study period are
sufficient to cover any bottlenecks and queues. In addition, because incident
data are supplied in the form of a facility crash rate, the VMT associated
with the base data set are calculated so that incident probabilities can be
calculated in a subsequent step. In this case, 71,501 vehicle miles of travel
occur on the facility over the 3-h base study period. The performance
measures normally output by the Chapter 10 methodology are compiled for
each combination of segment and analysis period during the study period and
stored for later use. Of particular note, the facility operates just under
capacity, with a maximum demand-to-capacity ratio of 0.99 in Segments 7–
10.


_Incorporating Demand Variability_

Exhibit 25-100 provides demand ratios relative to AADT by month and
day derived from a permanent traffic recorder on the facility. The demand


values for the seed file were collected on a Tuesday in November.


**Exhibit 25-100: Example Problem 7: Demand Ratios Relative to AADT**


**Month** **Monday** **Tuesday** **Wednesday** **Thursday** **Friday**

January 0.822 0.822 0.839 0.864 0.965
February 0.849 0.849 0.866 0.892 0.996
March 0.921 0.921 0.939 0.967 1.080
April 0.976 0.976 0.995 1.025 1.145
May 0.974 0.974 0.993 1.023 1.142
June 1.022 1.022 1.043 1.074 1.199
July 1.133 1.133 1.156 1.191 1.329
August 1.033 1.033 1.054 1.085 1.212
September 1.063 1.063 1.085 1.117 1.248
October 0.995 0.995 1.016 1.046 1.168
November 0.995 0.995 1.016 1.046 1.168
December 0.979 0.979 0.998 1.028 1.148


_Incorporating Weather Variability_

In the absence of facility-specific weather data, the default weather data
for the metropolitan area closest to the facility are used.

In the absence of local data, the default CAF and SAF for an FFS of 60
mi/h are used for each weather event. These values are applied in a later step
to each scenario involving a weather event. Exhibit 25-101 summarizes the
probabilities of each weather event by season, and Exhibit 25-102
summarizes the CAF, SAF, and event duration values associated with each
weather event.


**Exhibit 25-101: Example Problem 7: Weather Event Probabilities by Season**








|Weather Event|Weather Event Probability by Season (%)<br>Winter Spring Summer Fall<br>0.80 1.01 0.71 0.86<br>0.47 0.81 1.33 0.68<br>0.91 0.00 0.00 0.00<br>0.29 0.00 0.00 0.00<br>0.04 0.00 0.00 0.00<br>0.00 0.00 0.00 0.00<br>0.00 0.00 0.00 0.00<br>0.97 0.12 0.16 0.34|
|---|---|
|Medium rain<br>Heavy rain<br>Light snow<br>Light–medium snow<br>Medium–heavy snow<br>Heavy snow<br>Severe cold<br>Low visibility|Medium rain<br>Heavy rain<br>Light snow<br>Light–medium snow<br>Medium–heavy snow<br>Heavy snow<br>Severe cold<br>Low visibility|


Note: Winter = December, January, and February; spring = March, April, and May; summer
= June, July, and August; fall = September, October, and November.


**Exhibit 25-102: Example Problem 7: CAF, SAF, and Event Duration Values Associated**
**with Weather Events**

|Weather Event|CAF SAF Average Duration (min)|
|---|---|
|Medium rain<br>Heavy rain<br>Light snow<br>Light–medium snow<br>Medium–heavy snow<br>Heavy snow<br>Severe cold<br>Low visibility<br>Very low visibility<br>Minimal visibility<br>Nonsevere weather|0.93<br>0.95<br>40.2<br>0.86<br>0.93<br>33.7<br>0.96<br>0.92<br>93.1<br>0.94<br>0.90<br>33.4<br>0.91<br>0.88<br>21.7<br>0.78<br>0.86<br>7.3<br>0.92<br>0.95<br>0.0<br>0.90<br>0.95<br>76.2<br>0.88<br>0.94<br>0.0<br>0.90<br>0.94<br>145<br>1.00<br>1.00<br>N/A|



Note: N/A = not applicable.


_Incorporating Incident Variability_

For an existing freeway facility such as this one, detailed incident logs
would be desirable so that facility-specific monthly or seasonal probabilities
of various incident severities could be determined. However, in this case,
incident logs of sufficient detail are not available.

Therefore, incident probabilities and severities are estimated by the
alternative method of using local crash rates and ratios of incidents to
crashes, in combination with default values, by using Equation 25-77 through
Equation 25-79. The expected number of incidents during a study period
under a specified demand pattern is the product of the crash rate, the local
incident-to-crash ratio, the demand volume during the study period, and the
facility length. The crash rate is 150 crashes per 100 million VMT; the ratio
of incidents to crashes is given as 7. The resulting incident frequencies for
different months of the reliability reporting period are determined as shown
in Exhibit 25-103.


**Exhibit 25-103: Example Problem 7: Incident Frequencies by Month**


**Month** **Incident Frequency**

January 0.65
February 0.67
March 0.72
April 0.77
May 0.77
June 0.80
July 0.89
August 0.82
September 0.83
October 0.83
November 0.79
December 0.77


**Results and Discussion**

Exhibit 25-104 provides key reliability performance measure results for
this example problem. The number of replications for each scenario was
four, resulting in 240 scenarios. Exhibit 25-105 shows the generated
probability and cumulative distributions of travel time index (TTI) for this
example problem. A seed number of 1 was chosen to generate random
numbers in the computational engine.


**Exhibit 25-104: Example Problem 7: Summary Reliability Performance Measure Results**


**Reliability Performance Measure** **Value from All Scenarios**

_TTI_ 50 1.03
_TTI_ mean 1.30
PTI ( _TTI_ 95) 1.67
Maximum observed facility TTI ( _TTI_ max) 33.57
Misery index 5.76
Reliability rating 90.8%
Semi-standard deviation 2.05
Percentage VMT at TTI >2 2.95%


Note: PTI = planning time index; TTI = travel time index.


**Exhibit 25-105: Example Problem 7: VMT-Weighted TTI Probability and Cumulative**
**Distribution Functions**


(a) Probability Distribution Function


(b) Cumulative Distribution Function


**EXAMPLE PROBLEM 8: RELIABILITY ANALYSIS WITH**
**GEOMETRIC IMPROVEMENTS**


**The Facility**


In this example, the freeway facility from Example Problem 6 is widened
by a lane in Segments 7–11. These segments operated close to capacity in the
base scenario and were definitely over capacity in scenarios with severe
weather or incident conditions. The revised geometry also improves the
operation of weaving Segment 6, because no lane changes are required of
traffic entering at On-Ramp 2. Exhibit 25-106 provides a schematic of the
freeway facility.


**Exhibit 25-106: Example Problem 8: Freeway Facility**


**Data Inputs**

All the input data used in Example Problem 6 remain unchanged, except
for the number of lanes on the facility. The extra lane creates the possibility
of having a three-lane-closure incident scenario in the four-lane portion of
the facility.


**Results and Discussion**

Exhibit 25-107 provides key reliability performance measure results for
this example problem. The mean TTI across the reliability reporting period
decreases from 1.54 to 1.18, corresponding to a speed improvement from
38.96 to 50.8 mi/h—more than a 10% increase and perhaps enough to justify
the improvement, once non-reliability-related factors are taken into account.
Similar results occur for most other performance measures.


**Exhibit 25-107: Example Problem 8: Summary Reliability Performance Measure Results**


**Reliability Performance Measure** **Value from All Scenarios**

_TTI_ 50 1.02
_TTI_ mean 1.18


PTI ( _TTI_ 95) 1.17

Maximum observed facility TTI ( _TTI_ max) 33.5
Misery index 4.07
Reliability rating 97.56%
Semi-standard deviation 1.71
Percentage VMT at TTI >2 1.42%


Note: PTI = planning time index; TTI = travel time index.


**EXAMPLE PROBLEM 9: EVALUATION OF INCIDENT**
**MANAGEMENT**

This example problem illustrates the analysis of a nonconstruction
alternative that focuses on improved incident management strategies. In this
example, the size of the motorist response fleet is increased and
communication is improved between the various stakeholders (e.g., traffic
management center, emergency responders, and motorist response fleet),
allowing faster clearance of incidents than before.


**Data Inputs**

All the input data used in Example Problem 6 remain unchanged, except
for the assumed incident durations and standard deviations. The default
incident mean durations and standard deviations are reduced by 30% each
for all incident severity types. Note that these values have been created for
the purposes of this example problem and do not necessarily reflect results
that would be obtained in an actual situation.


**Results and Discussion**

The key congestion and reliability statistics for this example problem are
summarized in Exhibit 25-108. The mean TTI across the reliability reporting
period decreases from 1.35 to 1.20, corresponding to a speed improvement
from 44.4 to 50.0 mi/h—more than a 10% increase and perhaps enough to
justify the improvement, once non-reliability-related factors are taken into
account. Similar results occur for most other performance measures.


**Exhibit 25-108: Example Problem 9: Summary Reliability Performance Measure Results**


**Reliability Performance Measure** **Value from All Scenarios**


_TTI_ 50 1.03

_TTI_ mean 1.25
PTI ( _TTI_ 95) 1.59
Maximum observed facility TTI ( _TTI_ max) 30.7
Misery index 4.88
Reliability rating 91.36%
Semi-standard deviation 1.77
Percentage VMT at TTI >2 2.4%


Note: PTI = planning time index; TTI = travel time index.


**EXAMPLE PROBLEM 10: PLANNING-LEVEL RELIABILITY**
**ANALYSIS**

This example illustrates the planning-level reliability analysis
methodology described in Chapter 11. The method estimates the mean and
95th percentile TTI, as well as the percentage of trips occurring below a
speed of 45 mi/h.


**The Facts**

The segment under study has three lanes in the analysis direction, an FFS
of 75 mi/h, and a peak hour speed of 62 mi/h. The volume-to-capacity ratio
during the peak hour is 0.95.


**Solution**

The value of _TTI_ mean is calculated from Equation 11-1, and is a function
of the recurring delay rate _RDR_ and the incident delay rate _IDR_ . These rates
are calculated from Equation 11-2 and Equation 11-3, respectively.


_TTI_ mean can now be calculated as


_TTI_ 95 is calculated from Equation 11-4 as follows:


Finally, the percentage of trips made at a speed below 45 mi/h is
calculated with Equation 11-5.


**EXAMPLE PROBLEM 11: ESTIMATING FREEWAY COMPOSITE**
**GRADE OPERATIONS WITH THE MIXED-FLOW MODEL**


This example problem addresses a composite grade section on a six-lane
freeway. It illustrates how the mixed-flow model procedures can be applied
to the case of composite grades.


**The Facts**

   - Three segments with the following grades and lengths:

    - First segment: 1.5-mi basic segment on a 3% upgrade

    - Second segment: 2-mi basic segment on a 2% upgrade

    - Third segment: 1-mi basic segment on a 5% upgrade

   - 5% SUTs and 10% TTs

   - FFS of 65 mi/h

   - 15-min mixed-traffic flow rate is 1,500 veh/h/ln (PHF = 1.0)


**Comments**

Chapter 26, Basic Freeway and Highway Segments: Supplemental,
presents the procedure for estimating the speed on a single-grade basic
freeway segment using the mixed-flow model. The task here is to estimate the
speed by mode for each segment, along with the overall mixed-flow speed
and travel time for the composite grade.


**Step 1: Input Data**

All input data are specified above.


**Step 2: Capacity Assessment**

The CAF for mixed flow allows for the conversion of auto-only
capacities into mixed-traffic-stream capacities. It can be computed with
Equation 25-53.

For the first segment,


There are four terms in the equation. The CAF for auto-only conditions
_CAFao_ is assumed to be 1, because no auto adjustments are necessary.


_CAF for Truck Percentage_

The truck effect term is computed from Equation 25-54.


_CAF for Grade Effect_

The grade effect term is computed from Equation 25-55 and Equation 2556. Given that the total truck percentage is 15%, the coefficient _ρg,_ mix is
calculated as


and the CAF for grade effect for Segment 1 is calculated as


_Mixed-Flow CAF_

The mixed-flow CAF for Segment 1 can now be calculated from
Equation 25-53.


_Segment Capacity_

The mixed-flow capacity of segment 1 is computed from the segment’s
auto-only capacity and mixed-flow CAF. The auto-only capacity is
determined from an equation in Exhibit 12-6.


Segment 1’s mixed-flow capacity is then determined with Equation 2557.


Because the mixed-flow CAFs and capacities for Segments 2 and 3 can
be computed by following the same procedure, the results are presented
directly without showing the computational details.


As the mixed-flow demand of 1,500 veh/h/ln is less than the smallest of
the three segment capacities, 1,746 veh/h/ln, the analysis can proceed.


**Steps 3 to 6**

Steps 3 through 6 are repeated for each segment, as shown below.


_Segment 1_


_Step 3: Specify Initial Conditions_

Because this is the first segment, an FFS of 65 mi/h is used as the initial
truck kinematic spot travel time rate. The effect of traffic interactions on
truck speed is accounted for in Step 4.


_Step 4: Compute Truck Space-Based and Spot Travel Time_
_Rates_


**Kinematic Spot Rates.** The initial truck kinematic spot travel time rates
for both SUTs and TTs are 65 mi/h. These rates are located on the curves
representing a 3% upgrade starting from 75 mi/h (48 s/mi) in Exhibit 25-20
(SUTs) and Exhibit 25-21 (TTs).

The SUT and TT spot rates versus distance curves starting from 65 mi/h
will be applied to obtain _τf,SUT,kin,_ 1 and _τf,TT,kin,_ 1. In Exhibit 25-20, 65 mi/h
(55.4 s/mi) occurs about 4,100 ft into the 3% grade. After an SUT travels for
1.5 mi (7,920 ft) starting at an initial speed of 65 mi/h, its spot rate can be
read at 12,020 ft. That distance is outside the plot range, but Exhibit 25-20
shows SUTs reach a crawl speed of 59 s/mi (61 mi/h) at around 10,000 ft.
Therefore, the kinematic spot rate for SUTs at the end of the first segment
_τf,SUT,kin,_ 1 is 59 s/mi.

In Exhibit 25-21, 65 mi/h (55.4 s/mi) is found at about 2,100 ft. After a
TT travels for 1.5 mi (7,920 ft) from an initial speed of 65 mi/h, its spot rate
can be read at 12,020 ft, which is outside the plot range in Exhibit 25-21.
However, similar to SUTs, TTs approach their crawl speed at 10,000 ft,
namely 73 s/mi (49.3 mi/h).

Because this is the first segment, the initial truck kinematic rates
_τi,SUT,kin,_ 1 and _τi,TT,kin,_ 1 are equivalent to the free-flow rate of 55.4 s/mi.
Because _τi,SUT,kin,_ 1 is less than _τf,SUT,kin,_ 1 and _τi,TT,kin,_ 1 is less than _τf,TT,kin,_ 1,
both types of trucks decelerate on Segment 1, from 65 to 61 mi/h for SUTs
and from 65 to 49.3 mi/h for TTs.


**Kinematic Space-Based Rates.** Because this is the first segment, the
space-based speed at 0 ft is the FFS of 65 mi/h. Therefore, the 65-mi/h curve


is applied to obtain _τS,SUT,kin,_ 1 and _τS,TT,kin,_ 1.

The time for an SUT to travel 7,920 feet starting from 65 mi/h on a 3%
grade can be read from Exhibit 25-A7 and is 87 s. The corresponding travel
time for a TT can be read from Exhibit 25-A18 and is 99 s. The space mean
rate at 7,920 ft for an SUT _τS,SUT,kin,_ 65,7920 and a TT _τS,TT,kin,_ 65,7920 starting
from a FFS of 65 mi/h on a 3% grade can then be computed by Equation 2558:


**Auto-Only Speed for the Given Flow Rate.** The auto-only space mean
speed for the given flow rate is computed with Equation 25-63.


The choice of equation depends on whether demand volumes are greater
than or less than the breakpoint. An equation in Exhibit 12-6 is used to
compute the breakpoint. For an auto-only condition, the CAF defaults to 1.0.


As the demand volume of 1,500 veh/h/ln is greater than the breakpoint,
the second of the two auto-only speed equations will be used. This equation
requires knowing the auto-only capacity, which can be computed from
Exhibit 12-6.


Then


**Traffic Interaction Term.** The incremental traffic interaction term is
computed with Equation 25-62.


**Actual Spot Rates.** The actual spot travel time rates of SUTs and TTs at
the end of Segment 1 are computed from Equation 25-60 and Equation 25-61,
respectively.


The initial spot rates of SUTs and TTs in Segment 1 can also be
computed from Equation 25-60 and Equation 25-61.


**Actual Space-Based Rates.** Equation 25-60 and Equation 25-61 are
also used to calculate the actual space-based travel time rates for SUTs and
TTs. The traffic interaction term is the same as the term used for the spot rate
calculations.


_Step 5: Compute Spot and Space-Based Travel Time Rates for_
_Autos_

Equation 25-64 is used to compute the spot-based travel time rate for
automobiles on the basis of the kinematic truck spot rate at the end of the
segment.


When the initial auto spot travel time rate is computed, the trucks’
kinematic spot rates are the same as the FFS, so the last two terms are 0.
Therefore, Equation 25-64 can also be used to compute the initial auto spot
rate, with the last two terms equal to 0.


It was determined in Step 4 that trucks decelerate in the first segment, so
Equation 25-65 is used to compute the auto space-based rate on the basis of
the kinematic truck space-based rates.


_Step 6: Compute Mixed-Flow Space-Based Travel Time Rate_
_and Speed_

The mixed-flow travel rate _τ_ mix,1 and the mixed speed _S_ mix,1 are computed
with Equation 25-67 and Equation 25-68, respectively.


_Segment 2_


_Step 3: Specify Initial Conditions_


For the second segment, the initial truck kinematic spot travel time rates
are the final truck kinematic spot rates from the preceding segment. These are
59 s/mi (61.0 mi/h) for SUTs and 73 s/mi (49.3 mi/h) for TTs.


_Step 4: Compute Truck Space-Based and Spot Travel Time_
_Rates_


**Kinematic Spot Rates.** The initial truck kinematic spot travel time rates
for both SUTs and TTs were determined in Step 3.

In Exhibit 25-20, the initial SUT kinematic spot rate of 59 s/mi (61.0
mi/h) occurs on the curve for a 2% upgrade, starting from 30 mi/h (120 s/mi)
at approximately 4,000 ft along the curve. After an SUT travels for 2 mi
(10,560 ft), its spot rate can be read at 14,560 ft, which is outside the plot
range. However, Exhibit 25-20 shows SUTs approach their crawl speed of
67.9 mi/h (53 s/mi) on a 2% grade. Because the specified FFS is 65 mi/h,
SUTs will maintain a speed of 65 mi/h (55.4 s/mi) when the kinematic spot
speeds exceed 65 mi/h. Therefore, the SUT spot rate at the end of Segment 2,
_τf,SUT,kin,_ 2, is 55.4 s/mi.

In Exhibit 25-21, the initial TT kinematic spot rate of 73 s/mi (49.3 mi/h)
occurs on the curve for a 2% upgrade, starting from 20 mi/h (180 s/mi) at
approximately 3,360 ft. After a TT travels for 2 mi (10,560 ft), its spot rate
can be read at 13,920 ft, which is outside the plot range. However, Exhibit
25-21 shows TTs reach their crawl speed of 57.1 mi/h (63 s/mi) on a 2%
grade. Thus, the TT spot rate at the end of Segment 2, _τf,TT,kin,_ 2, is 63 s/mi.

On this segment, the final SUT and TT kinematic rates are greater than the
initial rates, so both truck types accelerate on the second grade. The
nomographs for the time versus distance relationships are applicable to both
cases where trucks are decelerating, and where they are accelerating.
Acceleration is evident if the time required to cover a given distance is
reducing as the distance increases.


**Kinematic Space-Based Rates.** The kinematic space-based speeds at 0
ft into Segment 2 equal the final kinematic spot speeds of Segment 1.

For SUTs, the final kinematic spot speed of Segment 1 was 61.0 mi/h (59
s/mi). As this speed is within 2.5 mi/h of 60 mi/h, Exhibit 25-A6 is used to
obtain the SUT kinematic space-based travel time rate _τS,SUT,kin,_ 2. The time


for an SUT to travel 10,000 ft starting from an FFS of 60 mi/h on a 2% grade
can be read from Exhibit 25-A6 and is 105 s.

For TTs, the final kinematic spot speed of Segment 1 was 49.3 mi/h (73
s/mi). As this speed is within 2.5 mi/h of 50 mi/h, Exhibit 25-A15 is applied
to obtain the TT kinematic space-based rate _τS,TT,kin,_ 2. The time for a TT to
travel 10,000 ft starting from an FFS of 50 mi/h on a 2% grade can be read
from Exhibit 25-A15 and is 125 s.

The space mean travel time rates for SUTs and TTs can now be computed
by Equation 25-58.


The SUT and TT kinematic rates at a distance of 2 mi (10,560 ft) can be
computed from Equation 25-59. The _δ_ values for SUTs (0.0104) and TTs
(0.0136) can be read from Exhibit 25-24 and Exhibit 25-25, respectively.
The rates are computed as follows:


**Auto-Only Speed for the Given Flow Rate.** The auto-only space mean
speed for the given flow rate is computed with Equation 25-63. The
breakpoint of the speed–flow curve was already determined to be 1,400
veh/h/ln, as part of the computations for the first segment. Thus,


**Traffic Interaction Term.** The incremental traffic interaction term is
computed by Equation 25-62.


**Actual Spot Rates.** The actual spot rates of SUTs and TTs at the end of
Segment 2 are computed from Equation 25-60 and Equation 25-61,
respectively.


Similarly, the space-based rates are


_Step 5: Compute Spot and Space-Based Travel Time Rates for_
_Autos_

Equation 25-64 is used to compute the spot-based travel time rate for
automobiles.


In this case, the auto spot rate of 60.1 s/mi is higher than the SUT spot
rate of 59.1 s/mi. As the auto spot rate should always be less than or equal to
the truck spot rate, the auto spot rate is set equal to 59.11 s/mi.

In Step 4, it was determined that trucks accelerate in Segment 2, so
Equation 25-66 is used to compute the auto space-based rate.


_Step 6: Compute Mixed-Flow Space-Based Travel Time Rate_
_and Speed_

The mixed-flow travel rate _τ_ mix,2 and the mixed speed _S_ mix,2 are computed
with Equation 25-67 and Equation 25-68, respectively.


_Segment 3_


_Step 3: Specify Initial Conditions_

The initial truck kinematic spot travel time rates for Segment 3 are the
final truck kinematic spot rates for Segment 2. These are 55.4 s/mi (65 mi/h)
for SUTs and 63.0 s/mi (57.1 mi/h) for TTs.


_Step 4: Compute Truck Space-Based and Spot Travel Time_
_Rates_


**Kinematic Spot Rates.** The initial truck kinematic spot travel time rates
for both SUTs and TTs were determined in Step 3.

In Exhibit 25-20, the initial SUT kinematic spot rate of 55.4 s/mi (65
mi/h) occurs on the curve for a 5% upgrade, starting from 75 mi/h (48 s/mi)
at approximately 1,500 ft along the curve. After an SUT travels 1 mi (5,280
ft), its spot rate can be read at 6,780 ft and is approximately 75 s/mi (48
mi/h). Thus, the SUT spot rate at the end of Segment 3 is 75 s/mi.

In Exhibit 25-21, the initial TT kinematic spot rate of 63 s/mi (57.1 mi/h)
occurs on the curve for a 5% upgrade, starting from 75 mi/h (48 s/mi) at
approximately 2,050 ft along the curve. After a TT travels 1 mi (5,280 ft), its
spot rate can be read at 7,330 ft and is approximately 103 s/mi (35.0 mi/h).
Thus, the TT spot rate at the end of Segment 3 is 103 s/mi.

In Segment 3, the initial kinematic rates for both truck types are less than
the final kinematic rates. Therefore, both truck types decelerate in Segment 3.


**Kinematic Space-Based Rates.** The kinematic space-based speeds at 0
ft into Segment 3 equal the final kinematic spot speeds of Segment 2.

The final kinematic spot speed of SUTs in Segment 2 was 65 mi/h (55.4
s/mi). Exhibit 25-A7 is therefore used to obtain the SUT kinematic space

based rate _τS,SUT,kin,_ 3. The travel time for SUTs at 5,280 ft, starting from 65
mi/h on a 5% grade, can be read from Exhibit 25-A7 and equals 67 s.

The final kinematic spot speed of TTs in Segment 2 was 57.2 mi/h (63.0
s/mi). As this value is within 2.5 mi/h of 55 mi/h, Exhibit 25-A16 is applied
to obtain the TT kinematic space-based rate _τS,TT,kin,_ 3. The travel time for
TTs at 5,280 ft, starting from an FFS of 55 mi/h on a 5% grade, can be read
from Exhibit 25-A16 and equals 89 s.

The space mean rate at 5,280 ft for SUTs and TTs can be computed by
Equation 25-58.


**Auto-Only Speed for the Given Flow Rate.** The auto-only space mean
speed for the given flow rate is computed with Equation 25-63. The
breakpoint of the speed–flow curve was already determined to be 1,400
veh/h/ln as part of the computations for the first segment. Thus


**Traffic Interaction Term.** The incremental traffic interaction term is
computed by Equation 25-62.


**Actual Spot Rates.** The actual spot rates of SUTs and TTs at the end of
Segment 2 are computed from Equation 25-60 and Equation 25-61,
respectively.


Similarly the space-based rates are:


_Step 5: Compute Spot and Space-Based Travel Time Rates for_
_Autos_

Equation 25-64 is used to compute the spot-based travel time rate for
automobiles.


In Step 4, it was determined that trucks decelerate in Segment 3, so
Equation 25-65 is used to compute the auto space-based rate.


_Step 6: Compute Mixed-Flow Space-Based Travel Time Rate_
_and Speed_

The mixed-flow travel rate _τ_ mix,3 and the mixed speed _S_ mix,3 are computed
using Equation 25-67 and Equation 25-68, respectively.


**Step 7: Overall Results**

Now that results have been developed for all three segments, the overall
performance of the composite grade can be computed. The mixed-flow travel
time for each segment is computed with Equation 25-69.


The overall mixed-flow travel time _t_ mix, _oa_ is the sum of the mixed-flow
travel times for all three segments and equals 294 s. Equation 25-70 can be
used to compute the mixed-flow speed.


Exhibit 25-109 shows the spot speeds of all the segments in the example.


**Exhibit 25-109: Example Problem 11: Spot Speeds of All Segments**


Exhibit 25-110 shows the space mean speeds of all the segments in the
example.


**Exhibit 25-110: Example Problem 11: Space Mean Speeds of All Segments**


Exhibit 25-111 shows the overall space mean speeds of all the segments
in the example.


**Exhibit 25-111: Example Problem 11: Overall Space Mean Speeds of All Segments**


## **12. REFERENCES**

1. _Highway Capacity Manual._ Transportation Research Board, National
Research Council, Washington, D.C., 2000.

2. Eads, B. S., N. M. Rouphail, A. D. May, and F. Hall. Freeway Facilities
Methodology in _Highway Capacity Manual 2000_ . In _Transportation_
_Research Record: Journal of the Transportation Research Board, No._
_1710_, Transportation Research Board, National Research Council,
Washington, D.C., 2000, pp. 171–180.

3. Hall, F. L., L. Bloomberg, N. M. Rouphail, B. Eads, and A. D. May.
Validation Results for Four Models of Oversaturated Freeway
Facilities. In _Transportation Research Record: Journal of the_
_Transportation Research Board, No. 1710_, Transportation Research
Board, National Research Council, Washington, D.C., 2000, pp. 161–
170.

4. _A Policy on Geometric Design of Highways and Streets_, 5th ed.
American Association of State Highway and Transportation Officials,
Washington, D.C., 2004.

5. Newell, G. F. A Simplified Theory of Kinematic Waves in Highway
Traffic. Part I: General Theory. _Transportation Research_, Vol. 27B, No.
4, 1993, pp. 281–287.

6. Newell, G. F. A Simplified Theory of Kinematic Waves in Highway
Traffic. Part II: Queuing at Freeway Bottlenecks. _Transportation_
_Research_, Vol. 27B, No. 4, 1993, pp. 289–303.

7. Newell, G. F. A Simplified Theory of Kinematic Waves in Highway
Traffic. Part III: Multidestination Flows. _Transportation Research_, Vol.
27B, No. 4, 1993, pp. 305–313.

8. Newman, L. _Freeway Operations Analysis._ Course Notes. University of
California Institute of Transportation Studies University Extension,
Berkeley, 1986.


9. Schoen, J. M., J. A. Bonneson, C. Safi, B. Schroeder, A. Hajbabaie, C.
H. Yeom, N. Rouphail, Y. Wang, W. Zhu, and Y. Zou. _Work Zone_
_Capacity Methods for the Highway Capacity Manual._ National
Cooperative Highway Research Program Project 3-107 final report,
preliminary draft. Kittelson & Associates, Inc., Tucson, Ariz., April
2015.

10. Hajbabaie, A., N. M. Rouphail, B. J. Schroeder, and R Dowling.
Planning-Level Methodology for Freeway Facilities. In _Transportation_
_Research Record: Journal of the Transportation Research Board_, No.
2483, Transportation Research Board of the National Academies,
Washington, D.C., 2015, pp. 47–56.

11. Elefteriadou, L., A. Kondyli, and B. St. George. _Estimation of_
_Capacities on Florida Freeways_ . Final Report. Gainesville, Fla., Sept.
2014.

12. Dowling, R., G. F. List, B. Yang, E. Witzke, and A. Flannery. _NCFRP_
_Report 31: Incorporating Truck Analysis into the_ Highway Capacity
Manual. Transportation Research Board of the National Academies,
Washington, D.C., 2014.

13. Washburn, S. S., and S. Ozkul. _Heavy Vehicle Effects on Florida_
_Freeways and Multilane Highways_ . Report TRC-FDOT-93817-2013.
Florida Department of Transportation, Tallahassee, Oct. 2013.

14. Ozkul, S., and S. S. Washburn. Updated Commercial Truck Speed
Versus Distance-Grade Curves for the _Highway Capacity Manual_ . In
_Transportation Research Record: Journal of the Transportation_
_Research Board_, No. 2483, Transportation Research Board of the
National Academies, Washington, D.C., 2015, pp. 91–101.

15. Hajbabaie, A., B. J. Schroeder, N. M. Rouphail, and S. Aghdashi.
_Freeway Facility Calibration Procedure_ . NCHRP 03-115 Working
Paper U-8b. ITRE at North Carolina State University, Raleigh, Aug.
2014. (Available in the Technical Reference Library section of HCM
Volume 4, http://hcmvolume4.org)

16. Hu, J., B. Schroeder, and N. Rouphail. Rationale for Incorporating
Queue Discharge Flow into _Highway Capacity Manual_ Procedure for
Analysis of Freeway Facilities. In _Transportation Research Record:_


_Journal of the Transportation Research Board, No. 2286_,
Transportation Research Board of the National Academies, Washington,
D.C., 2012, pp. 76–83.

17. Aghdashi, S., A. Hajbabaie, B. J. Schroeder, J. L. Trask, and N. M.
Rouphail. Generating Scenarios of Freeway Reliability Analysis:
Hybrid Approach. In _Transportation Research Record: Journal of the_
_Transportation Research Board_, No. 2483, Transportation Research
Board of the National Academies, Washington, D.C., 2015, pp. 148–
159.

18. Zegeer, J., J. Bonneson, R. Dowling, P. Ryus, M. Vandehey, W.
Kittelson, N. Rouphail, B. Schroeder, A. Hajbabaie, B. Aghdashi, T.
Chase, S. Sajjadi, R. Margiotta, and L. Elefteriadou. _Incorporating_
_Travel Time Reliability in the Highway Capacity Manual_ . SHRP 2
Report S2-L08-RW-1. Transportation Research Board of the National
Academies, Washington, D.C., 2014.

19. Federal Highway Administration. _Highway Economic Requirements_
_System—State Version (HERS-ST)_ . Technical Report. U.S. Department
of Transportation, Washington, D.C., 2005.

20. _Highway Safety Manual_, 2014 Supplement to the _Highway Safety_
_Manual_, 1st ed. American Association of State Highway and
Transportation Officials, Washington, D.C., 2014.


## **APPENDIX A: TRUCK PERFORMANCE CURVES**

This appendix provides travel time versus distance curves for SUTs for
initial speeds between 35 and 75 mi/h in 5-mi/h increments. Curves for SUTs
for 30- and 70-mi/h initial speeds are presented in Section 7 as Exhibit 2523 and Exhibit 25-22, respectively. The appendix also provides travel time
versus distance curves for TTs for initial speeds between 20 and 75 mi/h in
5-mi/h increments.


**Exhibit 25-A1: SUT Travel Time Versus Distance Curves for 35-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A2: SUT Travel Time Versus Distance Curves for 40-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A3: SUT Travel Time Versus Distance Curves for 45-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A4: SUT Travel Time Versus Distance Curves for 50-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A5: SUT Travel Time Versus Distance Curves for 55-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100.
Circles indicate where a truck reaches 60 mi/h, diamonds indicate 65 mi/h, and
squares indicate 70 mi/h.


**Exhibit 25-A6: SUT Travel Time Versus Distance Curves for 60-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100.
Diamonds indicate where a truck reaches 65 mi/h and squares indicate 70 mi/h.


**Exhibit 25-A7: SUT Travel Time Versus Distance Curves for 65-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 100.
Squares indicate where a truck reaches 70 mi/h.


**Exhibit 25-A8: SUT Travel Time Versus Distance Curves for 75-mi/h Initial Speed**


Note: Curves in this graph assume a weight-to-horsepower ratio of 100.


**Exhibit 25-A9: TT Travel Time Versus Distance Curves for 20-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A10: TT Travel Time Versus Distance Curves for 25-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A11: TT Travel Time Versus Distance Curves for 30-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A12: TT Travel Time Versus Distance Curves for 35-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A13: TT Travel Time Versus Distance Curves for 40-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A14: TT Travel Time Versus Distance Curves for 45-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A15: TT Travel Time Versus Distance Curves for 50-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Triangles indicate where a truck reaches 55 mi/h, circles indicate 60 mi/h, diamonds
indicate 65 mi/h, and squares indicate 70 mi/h.


**Exhibit 25-A16: TT Travel Time Versus Distance Curves for 55-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Circles indicate where a truck reaches 60 mi/h, diamonds indicate 65 mi/h, and
squares indicate 70 mi/h.


**Exhibit 25-A17: TT Travel Time Versus Distance Curves for 60-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Diamonds indicate where a truck reaches 65 mi/h and squares indicate 70 mi/h.


**Exhibit 25-A18: TT Travel Time Versus Distance Curves for 65-mi/h Initial Speed**


Notes: Curves in this graph assume a weight-to-horsepower ratio of 150.
Squares indicate where a truck reaches 70 mi/h.


**Exhibit 25-A19: TT Travel Time Versus Distance Curves for 70-mi/h Initial Speed**


Note: Curves in this graph assume a weight-to-horsepower ratio of 150.


**Exhibit 25-A20: TT Travel Time Versus Distance Curves for 75-mi/h Initial Speed**


Note: Curves in this graph assume a weight-to-horsepower ratio of 150.


