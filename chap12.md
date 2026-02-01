## **CHAPTER 12** **BASIC FREEWAY AND MULTILANE HIGHWAY** **SEGMENTS** **CONTENTS**

**1. INTRODUCTION**

Overview

Chapter Organization

Related HCM Content


**2. CONCEPTS**

Basic Freeway and Multilane Highway Segment Description

Flow Characteristics of Basic Freeway and Multilane Highway
Segments

Freeway Capacity Definitions

Capacity under Base Conditions

Speed–Flow Relationship

Basic Managed Lane Segment Concepts

Heavy Vehicle Concepts

Level of Service


**3. MOTORIZED VEHICLE CORE METHODOLOGY**

Scope of the Methodology

Required Data and Sources


Overview of the Methodology

Computational Steps


**4. EXTENSIONS TO THE METHODOLOGY**

Basic Managed Lane Segments

Bicycle Methodology for Multilane Highways


**5. APPLICATIONS**

Example Problems

Related Content in the HCMAG

Example Results

Planning and Preliminary Engineering Analysis

Design Analysis

Service Flow Rates, Service Volumes, and Daily Service Volumes

Use of Alternative Tools


**6. REFERENCES**

## **LIST OF EXHIBITS**


Exhibit 12-1: Basic Freeway Segment Types Illustrated

Exhibit 12-2: Multilane Highway Types Illustrated

Exhibit 12-3: Three Types of Flow on a Basic Freeway Segment

Exhibit 12-4: Basic Freeway and Multilane Highway Segment Capacity
Under Base Conditions

Exhibit 12-5: General Form for Speed–Flow Curves on Basic Freeway and
Multilane Highway Segments

Exhibit 12-6: Parameters for Speed–Flow Curves for Basic Freeway and
Multilane Highway Segments


Exhibit 12-7: Speed–Flow Curves for Basic Freeway Segments

Exhibit 12-8: Speed–Flow Curves for Multilane Highway Segments

Exhibit 12-9: Basic Managed Lane Segment Types

Exhibit 12-10: Continuous Access Managed Lane Speed–Flow Data With
and Without the General Purpose Lane Approaching Capacity

Exhibit 12-11: Estimated Lane Capacities for Basic Managed Lane Segments

Exhibit 12-12: Example Speed–Flow Relationships for a Continuous Access
Managed Lane Segment

Exhibit 12-13: Speed–Flow Curve Comparison for Managed Lane Segment
Types with 60-mi/h FFS

Exhibit 12-14: LOS Examples for Basic Freeway Segments

Exhibit 12-15: LOS Criteria for Basic Freeway and Multilane Highway
Segments

Exhibit 12-16: LOS Criteria and Speed–Flow Curves for Basic Freeway
Segments

Exhibit 12-17: LOS Criteria and Speed–Flow Curves for Multilane Highway
Segments

Exhibit 12-18: Required Input Data, Potential Data Sources, and Default
Values for Basic Freeway and Multilane Highway Segment Automobile
Analysis

Exhibit 12-19: Overview of Operational Analysis Methodology for Basic
Freeway and Multilane Highway Segments

Exhibit 12-20: Adjustment to FFS for Average Lane Width for Basic
Freeway and Multilane Highway Segments

Exhibit 12-21: Adjustment to FFS for Right-Side Lateral Clearance, _fRLC_
(mi/h), for Basic Freeway Segments

Exhibit 12-22: Adjustment to FFS for Lateral Clearances for Multilane
Highways

Exhibit 12-23: Adjustment to FFS for Median Type for Multilane Highways


Exhibit 12-24: Adjustment to FFS for Access Point Density for Multilane
Highways

Exhibit 12-25: PCEs for General Terrain Segments

Exhibit 12-26: PCEs for a Mix of 30% SUTs and 70% TTs

Exhibit 12-27: PCEs for a Mix of 50% SUTs and 50% TTs

Exhibit 12-28: PCEs for a Mix of 70% SUTs and 30% TTs

Exhibit 12-29: General Form for Speed–Flow Curves for Basic Managed
Lane Segments on Freeways

Exhibit 12-30: Parameters for Basic Managed Lane Segment Analysis

Exhibit 12-31: Bicycle LOS for Two-Lane and Multilane Highways

Exhibit 12-32: Illustrative Effect of Total Ramp Density and Right-Side
Lateral Clearance on Basic Freeway Segment FFS

Exhibit 12-33: Illustrative Effect of _v/c_ Ratio on Basic Freeway Segment
Speed

Exhibit 12-34: Illustrative Effect of Access Point Density, Lateral Clearance,
and Median Type on Multilane Highway Segment FFS

Exhibit 12-35: Illustrative Effect of Incidents and Inclement Weather on
Basic Freeway Segment FFS

Exhibit 12-36: Illustrative Effect of Inclement Weather and General Purpose
Lane Friction on Managed Lane FFS

Exhibit 12-37: Maximum Service Flow Rates for Basic Freeway Segments
Under Base Conditions

Exhibit 12-38: Maximum Service Flow Rates for Multilane Highway
Segments Under Base Conditions

Exhibit 12-39: Daily Service Volume Table for Urban Basic Freeway
Segments (1,000 veh/day)

Exhibit 12-40: Daily Service Volume Table for Rural Basic Freeway
Segments (1,000 veh/day)

Exhibit 12-41: Generalized Daily Service Volumes for Urban Multilane
Highways (1,000 veh/day)


Exhibit 12-42: Generalized Daily Service Volumes for Rural Multilane
Highways (1,000 veh/day)

Exhibit 12-43: Limitations of HCM Basic Freeway and Multilane Highway
Segments Procedure


## **1. INTRODUCTION**

**OVERVIEW**

This chapter presents methodologies for analyzing the capacity and level
of service (LOS) of basic freeway and multilane highway segments. These
segments are outside the influence of merging, diverging, and weaving
maneuvers. In the case of multilane highways, they are also outside the
influence of signalized intersections. Because of the similar operational
characteristics of basic freeway and multilane highway segments, they are
analyzed with the same methodology. The similarities include a common
form of the speed–flow relationship and the effects attributed to the number
of lanes, lane width, lateral clearance, and the presence of heavy vehicles.
The chapter also provides methods for analyzing basic managed lane
segments on freeways and bicycle LOS on multilane highways.

This chapter focuses on _uninterrupted flow_, which refers to accesscontrolled facilities, with access and egress being controlled through gradeseparated cross streets and ramp movements to access the facility. For
multilane highways, uninterrupted flow also exists when there are no traffic
control devices that interrupt traffic and where no platoons are formed by
upstream traffic signals. Typically, this condition occurs when the multilane
highway segment is 2 mi or more from the nearest traffic signal.

The methodologies in this chapter are limited to _uncongested flow_
conditions. Uncongested flow conditions require that the demand-to-capacity
ratio for the segment be less than or equal to 1.0. Uncongested flow on
freeways and multilane highways further means that there are no queuing
impacts on the segment from downstream bottlenecks. Chapter 10, Freeway
Facilities Core Methodology, provides an evaluation method for analyzing
oversaturated basic freeway segments. The _Highway Capacity Manual_
(HCM) does not currently provide a method for evaluating oversaturated
multilane highways other than to identify them as LOS F.


**CHAPTER ORGANIZATION**

Section 2 of this chapter presents the basic concepts of freeway and
multilane uninterrupted-segment operations, including the definition of base
conditions; differences in the treatment of basic freeway and multilane
segments; basic managed lane concepts; speed–flow relationships; and
demand, capacity, and LOS measures for automobile traffic.

Section 3 presents the base methodology for evaluating automobile
operations on basic freeway and multilane highway segments.

Section 4 extends the core method presented in Section 3 to applications
for managed lanes, including high-occupancy vehicle (HOV) and highoccupancy/toll (HOT) lanes (also called express or priced managed lanes)
with various types of separation from the general purpose lanes. This method
is based on findings from National Cooperative Highway Research Program
(NCHRP) Project 03-96 ( _1–3_ ). Additional extensions include the effect of
trucks and other heavy vehicles on capacity and LOS and a method for
evaluating bicycle LOS on multilane highways (with details provided in
Chapter 15, Two-Lane Highways).

Section 5 presents application guidance on using the results of basic
freeway and multilane highway segment analysis, including example results
from the methods, information on the sensitivity of results to various inputs,
and a service volume table for freeway and multilane highway segments.


**RELATED HCM CONTENT**

Other HCM content related to this chapter includes the following:

   - Chapter 3, Modal Characteristics, where the motorized vehicle
“Variations in Demand” subsection describes typical travel demand
patterns for freeway and multilane highway segments.

   - Chapter 4, Traffic Operations and Capacity Concepts, which
provides background for the speed, flow, density, and capacity terms
specific to freeway and multilane highway segments that are
presented in this chapter’s Section 2.

   - Chapter 10, Freeway Facilities Core Methodology, and Chapter 11,
Freeway Reliability Analysis, which use the basic freeway segment


methodology described in this chapter in analyzing a larger facility
comprising freeway basic, merge and diverge, weaving, and managed
lane segments over extended time periods.

- Chapter 11, Freeway Reliability Analysis, which provides a method
for evaluating freeway facilities with basic segments in a reliability
context. The chapter also provides default speed and capacity
adjustment factors that can be applied in this chapter’s methodology.

- Chapter 25, Freeway Facilities: Supplemental, which presents a
method for evaluating mixed truck and automobile traffic streams on
composite grades.

- Chapter 26, Freeway and Highway Segments: Supplemental, which
provides state-specific heavy vehicle percentages, presents a method
for evaluating mixed truck and automobile traffic streams on single
grades, describes capacity and speed adjustments for driver
populations unfamiliar with a roadway, provides guidance for
measuring freeway capacity in the field, describes a method for
evaluating freeway operations when connected and automated
vehicles are present in the traffic stream, and presents example
problems with step-by-step calculations using this chapter’s methods.

- Case Study 4, New York State Route 7, in the _HCM Applications_
_Guide_ in Volume 4, which demonstrates how this chapter’s methods
can be applied to the evaluation of an actual freeway facility.

- Section H, Freeway Analyses, and Section I, Multilane Highways, of
the _Planning and Preliminary Engineering Applications Guide to_
_the HCM,_ found in Volume 4, which describes how to incorporate
this chapter’s methods and performance measures into a planning or
preliminary engineering effort.


## **2. CONCEPTS**

**BASIC FREEWAY AND MULTILANE HIGHWAY SEGMENT**
**DESCRIPTION**

A basic freeway or multilane highway segment is outside the influence
area of any merge, diverge, or weaving segments and of any signalized
intersections. Exhibit 12-1 shows typical basic freeway segment cross
sections, and Exhibit 12-2 illustrates common types of multilane highways.


**Exhibit 12-1: Basic Freeway Segment Types Illustrated**


Source: © 2014 Google


(a) Eight-Lane Urban Freeway Segment


Source: © 2014 Google


(b) Six-Lane Rural Freeway Segment


**Exhibit 12-2: Multilane Highway Types Illustrated**


(a) Divided Suburban Multilane Highway Segment


(b) Undivided Suburban Multilane Highway Segment


(c) Suburban Multilane Highway Segment with Two-Way Left-Turn Lane


(d) Undivided Rural Multilane Highway Segment


Basic freeway segments generally have four to eight lanes (in both
directions) and posted speed limits between 50 and 75 mi/h. The median
type depends on right-of-way constraints and other factors.

Multilane highways generally have four to six lanes (in both directions)
and posted speed limits between 40 and 55 mi/h. In some states, speed limits
of 60 or 65 mi/h or higher are used on some multilane highways. These
highways may be undivided (with only a centerline separating the directions
of flow) or divided (with a physical median separating the directions of
flow), or they may have a two-way left-turn lane (TWLTL). Typically they
are located in suburban areas and lead into city centers or in high-volume
rural corridors, where they connect two cities or activity centers that
generate a substantial number of daily trips.

All analyses are applied to segments with uniform characteristics.
Uniform segments must have the same geometric and traffic characteristics,
including a constant demand flow rate.


**Influence Areas of Merge, Diverge, and Weaving Segments**

In general terms, the influence area of merge (on-ramp) segments extends
1,500 ft downstream of the merge point. The influence area of diverge (offramp) segments extends 1,500 ft upstream of the diverge point. The influence
area of weaving segments extends 500 ft upstream and downstream of the
gore-to-gore segment length. For undersaturated operations, these distances


define the areas most affected by merge, diverge, and weaving movements. A
complete discussion of these influence areas is provided in Chapter 10,
Freeway Facilities Core Methodology, with additional discussion in Chapter
13, Freeway Weaving Segments, and Chapter 14, Freeway Merge and
Diverge Segments.


**Influence of Breakdowns in Adjacent Freeway Segments**

The impact of breakdowns in any type of freeway segment on an adjacent
basic segment can be addressed with the methodologies of Chapter 10,
Freeway Facilities Core Methodology, and Chapter 11, Freeway Reliability
Analysis. Breakdown events are defined in more detail below.


**Influence of Traffic Signals on Multilane Highway Segments**

The influence area of traffic signals on multilane highways is typically
about 1 mi, which means that uninterrupted flow may exist if traffic signals
are spaced 2 mi or more apart. Many multilane highways will have periodic
signalized intersections, even if the average signal spacing is well over 2 mi.
In such cases, the multilane highway segments that are more than 2 mi away
from any traffic signals are analyzed with this chapter’s methodology.
Isolated signalized intersections along multilane highways should be
analyzed with the methodology of Chapter 19, Signalized Intersections.


**FLOW CHARACTERISTICS OF BASIC FREEWAY AND**
**MULTILANE HIGHWAY SEGMENTS**

Traffic flow within basic freeway segments can be highly dependent on
the conditions constricting flow at upstream and downstream bottleneck
locations. Such bottlenecks can be created by any or by a combination of the
following: merging, diverging, or weaving traffic; lane drops; maintenance
and construction activities; traffic accidents or incidents; objects in the
roadway; and geometric characteristics such as upgrades or sharp horizontal
curves. Bottlenecks can exist even when a lane is not fully blocked. Partial
blockages will cause drivers to slow and divert their paths. In addition, the
practice of rubbernecking near roadside incidents or accidents can cause
functional bottlenecks. Many nonrecurring congestion effects have a
facilitywide impact and therefore are considered in Chapter 10.


Uninterrupted flow on multilane highways is similar to that on basic
freeway segments. However, there are several important differences.
Because side frictions are present in varying degrees from uncontrolled
driveways and intersections, as well as from opposing flows on undivided
cross sections, speeds on multilane highways tend to be lower than those on
similar basic freeway segments. The basic geometry of multilane highways
tends to be more constrained than that of basic freeway segments, consistent
with lower speed expectations. Finally, isolated signalized intersections can
exist along multilane highways. The overall result is that speeds and
capacities on multilane highways are lower than those on basic freeway
segments with similar cross sections.

As was discussed in more detail in Chapter 4, Traffic Operations and
Capacity Concepts, traffic flow within a basic freeway or multilane highway
segment can be categorized as one of three general types: undersaturated,
queue discharge, and oversaturated.

   - _Undersaturated flow_ represents conditions under which the traffic
stream is unaffected by upstream or downstream bottlenecks.

   - _Queue discharge flow_ represents congested traffic flow that has just
passed through a bottleneck and is accelerating back to the drivers’
desired speeds. If no other downstream bottleneck exists, queue
discharge flow will be relatively stable until the queue is fully
discharged.

   - _Oversaturated flow_ represents the conditions within a queue that has
backed up from a downstream bottleneck. These flow conditions do
not reflect the prevailing conditions of the segment itself but rather
the consequences of a downstream problem. All oversaturated flow
is considered to be congested.

An example of each of the three types of flow discussed is illustrated in
Exhibit 12-3, which uses data from a freeway segment in California.


**Exhibit 12-3: Three Types of Flow on a Basic Freeway Segment**


Source: California Department of Transportation, 2008.
Note: I-405, Los Angeles, California.


**FREEWAY CAPACITY DEFINITIONS**

Freeway segment capacity is commonly understood to be a maximum
flow rate associated with the occurrence of some type of breakdown, which
results in lower speeds and higher densities. Previous research has shown
that when oversaturation begins, queues develop and vehicles discharge from
the bottleneck at a queue discharge rate that is usually lower than the
throughput rate before the breakdown. This is also known as the “capacity
drop phenomenon.” Several key terms related to freeway capacity are
defined below as they apply to this chapter. Details on the measurement of
breakdown and capacities are provided in Section 5 of Chapter 26, Freeway
and Highway Segments: Supplemental.


**Freeway Breakdown**

A freeway flow breakdown describes the transition from uncongested to
congested conditions. The formation of queues upstream of the bottleneck and
the reduced prevailing speeds make the breakdown evident.

In the HCM freeway methodology, a breakdown event on a freeway
bottleneck is defined as a sudden drop in speed of at least 25% below the


free-flow speed (FFS) for a sustained period of at least 15 min that results in
queuing upstream of the bottleneck.


**Recovery**

A freeway segment is considered to have recovered from the breakdown
event and the resulting oversaturated conditions when the average speeds (or
occupancies) reach prebreakdown conditions for a minimum duration of 15
min. The definition of recovery is therefore the inverse of the definition of
breakdown, requiring a recovery to near prebreakdown conditions
(operations above the speed threshold) for at least 15 min.

The HCM defines the breakdown recovery on a freeway bottleneck as a
return of the prevailing speed to within 10% of the FFS for a sustained
period of at least 15 min, without the presence of queuing upstream of the
bottleneck.


**Prebreakdown Flow Rate**

The prebreakdown flow rate is the flow rate that immediately precedes
the occurrence of a breakdown event. The literature suggests that this flow
rate does not have a fixed value, since evidence shows that breakdowns are
stochastic in nature and could occur following a range of flow rates. The
flow rate is typically expressed in units of passenger cars per hour per lane
(pc/h/ln) by converting trucks and other heavy vehicles into an equivalent
passenger car traffic stream.

In the HCM, the prebreakdown flow rate is defined as the 15-min
average flow rate immediately before the breakdown event. For the purpose
of this chapter, the prebreakdown flow rate is equivalent to the segment
capacity.


**Postbreakdown Flow Rate or Queue Discharge**

The postbreakdown flow rate is also referred to as the _queue discharge_
_flow rate_ or the average discharge flow rate. This flow rate is usually lower
than the prebreakdown flow rate, resulting in significant loss of freeway
throughput during congestion. Cases where the postbreakdown flow rate
exceeds the prebreakdown flow rate have also been observed, mostly when


the prebreakdown flow rate is low. Studies have indicated that the average
difference between the postbreakdown and the prebreakdown flow rates
varies from as little as 2% to as much as 20%, with a default value of 7%
recommended.

In the HCM the queue discharge rate is defined as the average flow rate
during oversaturated conditions (i.e., during the time interval after
breakdown and before recovery).


**CAPACITY UNDER BASE CONDITIONS**

The base conditions under which the full capacity of a basic freeway or
multilane highway segment is achieved include good weather, good
visibility, no incidents or accidents, no work zone activity, and no pavement
deterioration serious enough to affect operations. The term “base conditions”
presupposes the existence of these conditions. If any of these conditions does
not exist, the speed and capacity of the freeway segment can be adjusted
through this chapter’s methodology to reflect prevailing conditions. Base
conditions also include the following:

   - No heavy vehicles in the traffic stream,

   - A driver population mostly composed of regular users who are
familiar with the facility, and

   - 12-ft lane widths and adequate lateral clearances (different for
freeway and multilane highways).

The capacity of a basic freeway segment under base conditions varies
with the FFS. Exhibit 12-4 gives capacity values under base conditions for a
selection of FFS values. Interpolation between FFS values is permitted. In
all cases, capacity represents a maximum flow rate for a 15-min interval.


**Exhibit 12-4: Basic Freeway and Multilane Highway Segment Capacity Under Base**
**Conditions**


|FFS<br>(mi/h)|Capacity of Basic Freeway Capacity of Multilane Highway<br>Segments (pc/h/ln) Segments (pc/h/ln)|
|---|---|
|75<br>70<br>65|2,400<br>NA<br>2,400<br>2,300_a_<br>2,350<br>2,300_a_|


Notes: NA = not available.

_a_ Capacities for multilane highways with 65- and 70-mi/h FFS are extrapolated and
not based on field data.
It is reiterated that _these base capacities reflect ideal conditions on a_
_facility without any capacity-reducing effects._ For example, the base
capacities assume no heavy vehicles; no grades; and no additional friction
effects due to poor pavement conditions, narrow lanes, or lighting conditions.
Furthermore, the capacities shown in Exhibit 12-4 apply to a peak 15-min
period (expressed as hourly flow rates); capacities measured over a 1-h
period may be less than these values. Finally, the base capacities do not
include the effects of nonrecurring sources of congestion, such as severe
weather, incidents, or work zones. Therefore, calibration of the base capacity
to reflect local conditions is important, especially when a segment is
evaluated in the context of an extended freeway facility. For some
adjustments, the HCM method provides explicit guidance. In other cases,
available defaults for adjustment factors are limited, and these values should
therefore be obtained by using local data.

Chapters 10 and 11 provide additional information allowing capacity
values to be adjusted to reflect the impact of long- and short-term
construction and maintenance activities, adverse weather conditions,
accidents or incidents, and the use of active traffic and demand management.

The base capacity values represent national norms. Capacity varies
stochastically, and any given location could have a larger or smaller value.
Furthermore, capacity refers to the _average flow rate across all lanes_ . Thus,
a three-lane basic freeway segment with a 70-mi/h FFS would have an
expected base capacity of 3 × 2,400 = 7,200 pc/h. This flow would not be
uniformly distributed across all lanes. Thus, one or two lanes could have
stable base flows in excess of 2,400 pc/h/ln. Similarly, a two-lane (in one
direction) multilane highway segment with a 60-mi/h FFS would have an
expected capacity of 2 × 2,200 = 4,400 pc/h. This flow would not be
uniformly distributed. Thus, one lane could have stable flows in excess of
2,200 pc/h/ln.


Basic freeway and multilane highway segments reach their capacity at a
density of approximately 45 pc/mi/ln, although this value varies somewhat
from location to location. At this density, vehicles are spaced too closely to
dampen the impact of any perturbation in flow, such as a lane change or a
vehicle entering the roadway, without causing a disruption in flow that
propagates upstream.

In a freeway facility context (Chapter 10), a basic freeway segment
typically does not break down unless a work zone, incident, or geometric
constraint results in a reduction of the segment’s capacity relative to adjacent
segments. More commonly, the throughput of the basic freeway segment is
dictated by upstream or downstream merge, diverge, or weaving segments
that tend to govern the operations (and capacity) of the facility.


**SPEED–FLOW RELATIONSHIP**

Characteristics such as lane width, lateral clearance, median type, and
(in the case of multilane highways) access point density will affect the FFS
of the facility. Changes in the FFS further translate into different speed–flow
curves describing operations under base conditions at higher volume levels.

Under base conditions, speed–flow curves for uninterrupted flow on
basic freeway and multilane highway segments follow a common form:

   - _Constant speed range._ There is a range of flow rates (in passenger
cars per hour per lane) over which speed is constant. The range
extends from a flow rate of zero to a breakpoint value _BP_ . Over this
range, the speed is equal to the FFS.

   - _Decreasing speed range._ From _BP_ to the capacity _c_, speed decreases
from the FFS in a generally parabolic relationship.

   - _Capacity._ In all cases, capacity occurs when the traffic stream density
_D_ is 45 pc/mi/ln, indicated by the dashed line in Exhibit 12-5.

The general form of this relationship is illustrated in Exhibit 12-5, where
the _x_ -axis represents the adjusted 15-min demand flow rate _vp_ (pc/h/ln) and
the _y_ -axis represents the space mean speed _S_ of the traffic stream (mi/h). The
equation for the base speed–flow curve for every basic freeway and
multilane highway segment follows this form. In all cases, the value of


capacity is directly related to the FFS. For basic freeway segments, the value
of _BP_ is also directly related to the FFS. For multilane highway segments, the
breakpoint value is a constant value, occurring at 1,400 pc/h/ln.


**Exhibit 12-5: General Form for Speed–Flow Curves on Basic Freeway and Multilane**
**Highway Segments**


The general analytic form of the speed–flow relationship is given by
Equation 12-1, while the equations for determining the model parameters,
including the breakpoint and the capacity—both of which are based on FFS
—are given in Exhibit 12-6. The capacity adjustment (CAF) and speed
adjustment factors (SAF) shown in Exhibit 12-6 are calibration parameters
used to adjust for local conditions or to account for nonrecurring sources of
congestion, and they are discussed in the core methodology section of this
chapter. The CAF and SAF adjustments are only provided for basic freeway
segments, since no empirical research exists for equivalent capacity-reducing
effects on multilane highways.


**Equation 12-1:**


where _S_ is the mean speed of the traffic stream under base conditions (mi/h)
and other variables are as given in Exhibit 12-6.

The development and calibration of speed–flow curves for basic freeway
and multilane highway segments and the development of a common form for
representing these curves are described elsewhere ( _4_ - _7_ ). Basic speed–flow
curves have been developed for FFS values between 55 and 75 mi/h for
freeways and for FFS values between 45 and 70 mi/h for multilane highways
(however, the 65- and 70-mi/h curves should be used with caution since data
for those conditions are limited).


**Exhibit 12-6: Parameters for Speed–Flow Curves for Basic Freeway and Multilane**
**Highway Segments**


























|Param-<br>eter Definition and Units|Multilane Highway<br>Basic Freeway Segments Segments|
|---|---|
|_FFS_<br>Base segment free-flow<br>speed (mi/h)|Measured<br>OR predicted with Equation<br>12-2<br>Measured<br>OR predicted with<br>Equation 12-3|
|_FFSadj_<br>Adjusted free-flow speed<br>(mi/h)|_FFSadj_ =_ FFS_ ×_ SAF_<br>No adjustments|
|_SAF_<br>Speed adjustment factor<br>(decimal)|Locally calibrated<br>OR estimated with Chapter<br>11;<br>_SAF_ = 1.00 for base<br>conditions<br>1.00|
|_c_<br>Base segment capacity<br>(pc/h/ln)|_c_ = 2,200 + 10(_FFS_ – 50)<br>_c_ = 2,400<br>55 =_ FFS_ = 75<br>_c_ = 1,900 + 20(_FFS_ –<br>45)<br>_c_ = 2,300<br>45 =_ FFS_ = 70|
|_cadj_<br>Adjusted segment capacity<br>(pc/h/ln)|_cadj_ =_ c_ ×_ CAF_<br>No adjustments|
|_CAF_<br>Capacity adjustment factor<br>(decimal)|Locally calibrated<br>OR estimated with Chapter<br>11;<br>_CAF_ = 1.00 for base<br>conditions<br>1.00|
|_Dc_<br>Density at capacity<br>(pc/mi/ln)|45<br>45|
|||


|BP Breakpoint<br>(pc/h/ln)|BP = [1,000 + 40 × (75 – 1,400<br>adj<br>FFS )] × CAF 2<br>adj|
|---|---|
|_a_<br>Exponent calibration<br>parameter (decimal)|2.00<br>1.31|


The largest difference in the speed–flow curves for basic freeway and
multilane highway segments is in the breakpoint. For freeways, the
breakpoint varies with FFS—specifically, the breakpoint _increases_ as the
FFS _decreases_ . This suggests that at lower values of FFS, drivers will
maintain the FFS through higher flow levels. For multilane highways, the
breakpoint is a constant. Exhibit 12-7 and Exhibit 12-8 show the base speed–
flow curves for basic freeway and multilane highway segments, respectively,
for 5-mi/h increments of FFS.


**Exhibit 12-7: Speed–Flow Curves for Basic Freeway Segments**


**Exhibit 12-8: Speed–Flow Curves for Multilane Highway Segments**


Note: Dashed curves are extrapolated and not based on field data.


**BASIC MANAGED LANE SEGMENT CONCEPTS**


**Types of Managed Lane Segments**

Managed lane segments may include HOV lanes, HOT lanes, or express
toll lanes. The vehicle composition, driver type, FFS, capacity, and driver
behavior characteristics of managed lane traffic streams are different from
those of general purpose lanes. In addition, interaction occurs between the
two traffic streams, especially when there is no physical barrier between the
managed and the general purpose lanes ( _1_ - _3_ ).

Five types of basic managed lane segments are identified, on the basis of
the number of managed lanes and the type of separation from the general
purpose lanes. The speed–flow characteristics of each basic managed lane
segment type are different. The five segment types are illustrated in Exhibit
12-9 and consist of the following:

1. _Continuous access:_ Skip-stripe or solid single line–separated, single
lane;


2. _Buffer 1:_ Buffer-separated, single lane;

3. _Buffer 2:_ Buffer-separated, multiple lanes;

4. _Barrier 1:_ Barrier-separated, single lane; and

5. _Barrier 2:_ Barrier-separated, multiple lanes.


**Basic Managed Lane Segment Capacity**

The capacity of managed lanes can be difficult to ascertain because they
are often designed to operate at high levels of service and below capacity.
While managed lanes do fail, empirical data on their true capacity values are
limited. HOT lane users are provided with an incentive to pay for the use of
the lane in return for achieving reliable travel times. Research ( _1_ - _3_ ) has
documented the maximum observed 15-min hourly flow rates (without any
breakdowns observed) on basic managed lane segments, and these values are
documented in this chapter as the “capacity.” Actual managed lane segment
capacity, therefore, may be underestimated in some cases. Users of the HCM
are encouraged to calibrate parameters to reflect local conditions. In this
chapter’s methodologies, the speed–flow curves for both managed and
general purpose lanes can be modified to account for local measurements of
capacity, FFS, or both.

The capacity of a basic managed lane segment depends on the number of
lanes on the segment. A single-lane managed lane segment does not offer the
opportunity to pass slower vehicles, which greatly reduces its capacity and
affects its speed–flow relationship. Capacity is also highly dependent on the
type of separation between the managed and general purpose lanes, with
barrier-separated managed lanes less susceptible to operational conditions in
the general purpose lanes than other types of managed lanes (continuous
access, marking-only, and buffer-separated). This effect is discussed in more
detail below.


**Exhibit 12-9: Basic Managed Lane Segment Types**


Continuous Access


Buffer 1


Buffer 2


Barrier 1



Source: ©2014 Google.
Note: I-5, Seattle, Washington.


Source: ©2014 Google.
Note: I-394, Minneapolis, Minnesota.


Source: ©2014 Google.
Note: I-110, Los Angeles, California.


Barrier 2



Source: ©2014 Google.
Note: I-5, Orange County, California.


Source: ©2014 Google.
Note: I-5, Seattle, Washington.



Exhibit 12-10 shows how the speed–flow relationship at high flows
diverges for a continuous access basic managed lane segment once the
neighboring general purpose lanes approach capacity. Divergence typically
occurs when the general purpose lane density exceeds 35 pc/mi/ln, which is
the threshold for entering LOS E. This interaction starts even at low flow
rates on the managed lane at about 500 pc/h/ln. Managed lanes with barrier
separation, on the other hand, operate virtually the same as general purpose
lanes and do not appear to be sensitive to high densities in the general
purpose lanes.


**Exhibit 12-10: Continuous Access Managed Lane Speed–Flow Data With and Without**
**the General Purpose Lane Approaching Capacity**


Note: GP = general purpose lane.


Exhibit 12-11 provides estimated capacities for basic managed lane
segments as a function of the FFS and separation from the general purpose
lanes. As mentioned above, these values represent the maximum observed
flow rates from a national study of managed lane segments ( _1_ - _3_ ) but are not
necessarily associated with a density of 45 pc/h/ln.


**Exhibit 12-11: Estimated Lane Capacities for Basic Managed Lane Segments**







|FFS<br>(mi/h)|Estimated Lane Capacities (pc/h/ln) by Basic Managed Lane Segment Type<br>Continuous Access Buffer 1 Buffer 2 Barrier 1 Barrier 2|
|---|---|
|75<br>70<br>65<br>60<br>55|1,800<br>1,700<br>1,850<br>1,750<br>2,100<br>1,750<br>1,650<br>1,800<br>1,700<br>2,050<br>1,700<br>1,600<br>1,750<br>1,650<br>2,000<br>1,650<br>1,550<br>1,700<br>1,600<br>1,950<br>1,600<br>1,500<br>1,650<br>1,550<br>1,900|


An example illustration of the resulting speed–flow curves for a managed
lane segment with continuous access is shown in Exhibit 12-12. An
illustration and comparison of the speed–flow relationships for different


types of managed lanes are shown in Exhibit 12-13. The parameters used to
obtain these curves are presented later in Exhibit 12-30.

In both exhibits, the _frictional effect_ refers to a managed lane that is
affected by elevated density in the general purpose lanes (i.e., densities
greater than 35 pc/mi/ln). This frictional effect only applies to some of the
managed lane types and specifically does not occur for barrier-separated
managed lanes or two-lane managed lanes with buffer separation.


**Exhibit 12-12: Example Speed–Flow Relationships for a Continuous Access Managed**
**Lane Segment**


**Exhibit 12-13: Speed–Flow Curve Comparison for Managed Lane Segment Types with**
**60-mi/h FFS**


**HEAVY VEHICLE CONCEPTS**

The traffic performance of heavy vehicles is significantly different from
that of automobiles. The differences relate to vehicle acceleration and
deceleration characteristics, as reflected in their weight-to-power ratios and
lengths. Two categories of heavy vehicles are defined: single-unit trucks
(SUTs) and tractor-trailers (TTs). Buses and recreational vehicles are
treated as SUTs in the HCM. Chapter 3, Modal Characteristics, provides a
more detailed discussion of the types of heavy vehicles and compares the
HCM and Federal Highway Administration (FHWA) vehicle classification
schemes. FHWA Classifications 4 and 5 are treated as SUTs by the HCM,
while FHWA Classifications 6 and higher are considered as TTs.

Two distinct methodologies are offered to assess the effect of heavy
vehicles on capacity and LOS on freeways in the HCM:

1. Traditional passenger car equivalency (PCE) factors that allow the
analyst to convert a mixed stream of cars and trucks to a single


uniform PCE stream for purpose of analysis; and

2. A mixed-flow model that directly assesses the capacity, speed, and
density of traffic streams that include a significant percentage of
heavy vehicles operating on a single or composite grade.

This chapter’s core methodology uses the PCE approach, while the
mixed-flow model is presented in Volume 4 as an extension of the
methodology. The mixed-flow model for single grades is found in Chapter
26, Freeway and Highway Segments: Supplemental, while the model for
composite grades is found in Chapter 25, Freeway Facilities: Supplemental.
The mixed-flow model form is fully consistent with Equation 12-1 and uses
supporting equations to estimate a SAF, CAF, breakpoint, density at capacity,
speed at capacity, and exponent calibration parameter. When the mixed-flow
models are used, no PCEs are needed, since the passenger car, SUT, and TT
volumes are used directly in the estimation of mixed-flow speed and density.

In fact, the mixed-flow method was used to generate the PCE tables as
well as an equation for estimating the PCE value for any traffic mix of SUTs
and TTs, as shown in Section 3. These PCE tables, and the associated
equations in Volume 4, can be used to assess the LOS for a given mixed-flow
segment without the direct use of the mixed-flow model. The PCE values are
predicated on equivalency between the mixed-flow rate at capacity (in
vehicles per hour per lane) and the flow rate of the equivalent automobileonly traffic stream (in passenger cars per hour per lane). The PCE tables
assume the following splits between SUTs and TTs: 30% SUTs and 70%
TTs, 50% SUTs and 50% TTs, and 70% SUTs and 30% TTs. The PCE
equation on which the tables are based allows other truck mixes to be
assessed.

If the PCE tables are used by themselves, the resulting speeds and
densities for the equivalent automobile-only traffic stream may differ from
those characterizing the mixed-flow condition. For most freeway analyses,
PCE tables are sufficient and provide a reasonable approximation of the
truck effects. However, if truck percentages are high or grades are
significant, the mixed-flow model is expected to give a more accurate result.
If estimates of the actual mixed-flow speeds and densities are desired, the
mixed-flow model in Volume 4 should be used. If the basic freeway segment


is analyzed as part of a freeway facility with the methodology in Chapter 10,
a PCE approximation is typically appropriate and recommended.


**LEVEL OF SERVICE**

LOS on basic freeway and multilane highway segments is defined by
density. Although speed is a major concern of drivers related to service
quality, describing LOS on the basis of speed would be difficult, since it
remains constant up to high flow rates [i.e., 1,000 to 1,800 pc/h/ln for basic
freeway segments (depending on the FFS) and 1,400 pc/h/ln for multilane
highway segments]. Density describes a motorist’s proximity to other
vehicles and is related to a motorist’s freedom to maneuver within the traffic
stream. Unlike speed, density is sensitive to flow rates throughout the range
of flows. Exhibit 12-14 illustrates the six levels of service defined for basic
freeway segments.


**Exhibit 12-14: LOS Examples for Basic Freeway Segments**


LOS A


LOS B


LOS C


LOS D


LOS E


LOS F


**LOS Described**

LOS A describes free-flow operations. FFS prevails on the freeway or
multilane highway, and vehicles are almost completely unimpeded in their
ability to maneuver within the traffic stream. The effects of incidents or point
breakdowns are easily absorbed.

LOS B represents reasonably free-flow operations, and FFS on the
freeway or multilane highway is maintained. The ability to maneuver within
the traffic stream is only slightly restricted, and the general level of physical
and psychological comfort provided to drivers is still high. The effects of
minor incidents are still easily absorbed.

LOS C provides for flow with speeds near the FFS of the freeway or
multilane highway. Freedom to maneuver within the traffic stream is
noticeably restricted, and lane changes require more care and vigilance on


the part of the driver. Minor incidents may still be absorbed, but the local
deterioration in service quality will be significant. Queues may be expected
to form behind any significant blockages.

LOS D is the level at which speeds begin to decline with increasing
flows, with density increasing more quickly. Freedom to maneuver within the
traffic stream is seriously limited, and drivers experience reduced physical
and psychological comfort levels. Even minor incidents can be expected to
create queuing, because the traffic stream has little space to absorb
disruptions.

LOS E describes operation at or near capacity. Operations on the
freeway or multilane highway at this level are highly volatile because there
are virtually no usable gaps within the traffic stream, leaving little room to
maneuver within the traffic stream. Any disruption to the traffic stream, such
as vehicles entering from a ramp or an access point or a vehicle changing
lanes, can establish a disruption wave that propagates throughout the
upstream traffic stream. Toward the upper boundary of LOS E, the traffic
stream has no ability to dissipate even the most minor disruption, and any
incident can be expected to produce a serious breakdown and substantial
queuing. The physical and psychological comfort afforded to drivers is poor.

LOS F describes unstable flow. Such conditions exist within queues
forming behind bottlenecks. Breakdowns occur for a number of reasons:

   - Traffic incidents can temporarily reduce the capacity of a short
segment, so that the number of vehicles arriving at a point is greater
than the number of vehicles that can move through it.

   - Points of recurring congestion, such as merge or weaving segments
and lane drops, experience very high demand in which the number of
vehicles arriving is greater than the number of vehicles that can be
discharged.

   - In analyses using forecast volumes, the projected flow rate can
exceed the estimated capacity of a given location.

In all cases, breakdown occurs when the ratio of existing demand to
actual capacity, or of forecast demand to estimated capacity, exceeds 1.00.
LOS F operations within a queue are the result of a breakdown or bottleneck
at a downstream point. In practical terms, the point of the breakdown has a


_d_ / _c_ ratio greater than 1.00 and is also labeled LOS F, although actual
operations at the breakdown point and immediately downstream may actually
reflect LOS E conditions. Whenever queues due to a breakdown exist, they
have the potential to extend upstream for considerable distances. In that case,
the upstream conditions (in the queue) will likely operate at LOS F speeds
and densities, even if the segment-level predictions are LOS E or better.
Therefore, for accurate estimation of the operational performance of these
queue spillback effects, a freeway facility analysis should be conducted by
using the procedure in Chapter 10 whenever one or more segment demands
exceed capacity.


**LOS Criteria**

A basic freeway or multilane highway segment can be characterized by
three performance measures: density in passenger cars per mile per lane,
space mean speed in miles per hour, and the ratio of demand flow rate to
capacity ( _v_ / _c_ ). Each of these measures is an indication of how well traffic is
being accommodated by the basic freeway segment.

Because speed is constant through a broad range of flows and the _v_ / _c_
ratio is not directly discernible to road users (except at capacity), the service
measure for basic freeway and multilane highway segments is density.
Exhibit 12-15 shows the criteria.


**Exhibit 12-15: LOS Criteria for Basic Freeway and Multilane Highway Segments**


**LOS** **Density (pc/mi/ln)**

A ≤11
B >11–18
C >18–26
D >26–35
E >35–45
F Demand exceeds capacity OR density > 45


The LOS thresholds for basic freeway and multilane highway segments
are the same for urban and rural locations, as defined by the FHWA
smoothed or adjusted urbanized boundaries ( _8_ ). However, note that a
freeway facilities analysis (Chapter 10) defines different LOS thresholds for
urban and rural _facilities._


For all levels of service, the density boundaries on basic freeway
segments are the same as those for multilane highways. Traffic
characteristics are such that the maximum flow rates at any given LOS are
lower on multilane highways than on similar basic freeway segments.

The specification of maximum densities for LOS A to D is based on the
collective professional judgment of the members of the Transportation
Research Board’s Committee on Highway Capacity and Quality of Service.
The upper value shown for LOS E (45 pc/mi/ln) is the maximum density at
which sustained flows at capacity are expected to occur. In effect, as
indicated in the speed–flow curves of Exhibit 12-7, when a density of 45
pc/mi/ln is reached, flow is at capacity, and the _v_ / _c_ ratio is 1.00.

In the application of this chapter’s methodology, however, LOS F is
identified when demand exceeds capacity because the analytical
methodology _does not allow_ the determination of density when demand
exceeds capacity. Although the density will be greater than 45 pc/h/ln, the
methodology of Chapter 10, Freeway Facilities Core Methodology, must be
applied to determine a more precise density for such cases.

Exhibit 12-16 illustrates the range of densities for a given LOS on the
base speed–flow curves for basic freeway segments. On a speed–flow plot,
density is a line of constant slope starting at the origin. The LOS boundaries
were defined to produce reasonable ranges for each LOS letter. Exhibit 1217 shows the same relationships applied to multilane highway segments. The
two dashed lines in the latter exhibit correspond to speed–flow relationships
that were extrapolated from other results but that have not been calibrated
from field data.


**Exhibit 12-16: LOS Criteria and Speed–Flow Curves for Basic Freeway Segments**


**Exhibit 12-17: LOS Criteria and Speed–Flow Curves for Multilane Highway Segments**


Note: Dashed curves are extrapolated and not based on field data.


## **3. MOTORIZED VEHICLE CORE METHODOLOGY**

This chapter’s methodology can be used to analyze the capacity, LOS,
and lane requirements of basic freeway or multilane highway segments and
the effects of design features on their performance. The methodology is based
on the results of an NCHRP study ( _4_ ), which has been partially updated ( _5_ ).
A number of significant publications were also used in the development of
the methodology ( _6_, _7_, _9–17_ ).


**SCOPE OF THE METHODOLOGY**

The methodology described in this section is applicable to general
purpose uninterrupted-flow, undersaturated basic freeway and multilane
segments. Oversaturated conditions on basic freeway segments can be
analyzed with the method described in Chapter 10, Freeway Facilities Core
Methodology. Extensions of the methodology described in Section 4 address
basic managed lane segments and bicycle LOS on multilane highways.
Chapter 26, Freeway and Highway Segments: Supplemental, presents a
method to analyze freeway operations on segments with significant truck
presence, a prolonged single upgrade, or both.


**Spatial and Temporal Limits**

Determining capacity or LOS requires uniform traffic and roadway
conditions on the analysis segment. Thus, any point where roadway or traffic
conditions change must mark a boundary of the analysis segment.

At every ramp–freeway (or ramp–multilane highway) junction, the
demand volume changes as some vehicles enter or leave the traffic stream.
Thus, any ramp junction should mark a boundary between adjacent basic
freeway or multilane highway segments.

In addition to ramp–freeway junctions, the following conditions generally
dictate that a boundary be established between basic freeway or multilane


highway segments:

   - Change in the number of lanes (cross section);

   - Changes in lane width or lateral clearance;

   - Grade change of 2% or more on a specific or composite grade;

   - Change in terrain category (for general terrain segments);

   - Presence of a traffic signal, STOP sign, or roundabout along a
multilane highway;

   - Significant change in the access point density or total ramp density;

   - Presence of a bottleneck condition;

   - Change in posted speed limit; or

   - Presence of an access point at which a significant number or
percentage of vehicles enters or leaves a multilane highway.

The last item in this list is not directly involved in the analysis of a basic
freeway or multilane highway segment but would probably reflect changes in
ramp or access point density or other features.

The analysis period for any freeway or multilane highway analysis is
generally the peak 15-min period within the peak hour. Any 15-min period
can be analyzed, however.

If demand volumes are used, demand flow rates are estimated through use
of the peak hour factor (PHF). When 15-min volumes are measured directly,
the analysis period within the hour that has the highest volumes is selected,
and flow rates are the 15-min volumes multiplied by 4. For subsequent
computations in the methodology, the PHF is set to 1.00.


**Performance Measures**

The core motorized vehicle methodology generates the following
performance measures:

   - Capacity,

   - FFS,

   - Demand- and volume-to-capacity ratios,


   - Space mean speed,

   - Average density, and

   - Motorized vehicle LOS.


**Limitations of the Methodology**

This chapter’s methodologies for basic freeway segments and multilane
highways do not apply to or take into account (without modification by the
analyst) the following:

   - Lane controls (to restrict lane changing);

   - Extended bridge and tunnel segments;

   - Segments near a toll plaza;

   - Facilities with a FFS more than 75 mi/h for basic freeway segments
or more than 70 mi/h for multilane highways;

   - Facilities with a base FFS less than 55 mi/h for freeways and less
than 45 mi/h for multilane highways, although lower FFS values can
be achieved for freeway segments by calibrating a SAF;

   - Posted speed limit and enforcement practices;

   - Presence of intelligent transportation systems (ITS) related to vehicle
or driver guidance;

   - Capacity-enhancing effects of ramp metering;

   - The influence of downstream queuing on a segment;

   - Operational effects of oversaturated conditions; and

   - Operational effects of construction operations.

The last four items in the list of limitations above are addressed in a
freeway facility analysis context, as described in Chapter 10. The following
are additional limitations for this chapter’s multilane highway methodology:

   - The effect of lane drops and lane additions at the beginning or end of
multilane highway segments;


   - Possible queuing impacts when a multilane highway segment
transitions to a two-lane highway segment;

   - The negative impacts of poor weather conditions, traffic accidents or
incidents, railroad crossings, or construction operations on multilane
highways;

   - Differences between various types of median barriers and the
difference between the impacts of a median barrier and a TWLTL;

   - Significant presence of on-highway parking;

   - Presence of bus stops that have significant use; and

   - Significant pedestrian activity.

The last three factors are more representative of an urban or suburban
arterial, but they may also exist on multilane highway facilities with more
than 2 mi between traffic signals. When these factors are present on
uninterrupted-flow segments of multilane highways, the methodology does
not deal with their impact on flow. In addition, this methodology cannot be
applied to highways with a total of three lanes in both directions, which
should be analyzed as two-lane highways with periodic passing lanes by
using the methods of Chapter 15.

Uninterrupted-flow multilane highway facilities that allow access solely
through a system of on-ramps and off-ramps from grade separations or
service roads should be analyzed as freeways. Note that some ramp access
or egress points may be present on a multilane highway where most access or
egress points are at-grade junctions of some type.

To address most of the limitations listed above, the analyst would have to
utilize alternative tools or draw on other research information and develop
special-purpose modifications of this methodology. Operational effects of
oversaturated conditions, incidents, work zones, and weather and lighting
conditions can be evaluated with the methodology of Chapter 10 and
adjustment factors for capacity and FFS found in Chapter 11. Operational
effects of active traffic and demand management (ATDM) measures can be
evaluated by using the procedures in Chapter 11, Freeway Reliability
Analysis. A broader overview of ATDM strategies is presented in Chapter
37, ATDM: Supplemental.


**Alternative Tools**


_Strengths of HCM Procedures_

This chapter’s procedures were developed on the basis of extensive
research supported by a significant quantity of field data. They have evolved
over a number of years and represent an expert consensus.

Specific strengths of the HCM basic freeway and multilane highway
segment methodology include the following:

   - It provides a detailed methodology for measuring or estimating FFS.
This methodology is based on various geometric characteristics. In
simulation packages, FFS (or an equivalent, such as desired speed) is
usually an input.

   - It considers geometric characteristics (such as lane widths), which
are rarely, if ever, incorporated into simulation algorithms.

   - It provides explicit capacity estimates. Simulation packages do not
provide capacity estimates directly. Capacity estimates can only be
obtained from simulators through multiple runs with oversaturated
conditions. The user can modify simulated capacities by modifying
specific input values such as the minimum acceptable headway.

   - It produces a single deterministic estimate of traffic density, which is
important for some purposes such as development impact review.


_Limitations of HCM Procedures That Might Be Addressed by_
_Alternative Tools_

Basic freeway segments can be analyzed by using a variety of stochastic
and deterministic simulation packages that include freeways. These packages
can be useful in analyzing the extent of congestion when there are failures
within the simulated facility range and when interaction with other freeway
segments and other facilities is present. Less is known about the ability of
simulation models to characterize multilane highway operations.


_Additional Features and Performance Measures Available from_
_Alternative Tools_


This chapter provides a methodology for estimating the capacity, speed,
and density of a basic freeway segment, given the segment’s traffic demand
and characteristics. Alternative tools offer additional performance measures,
including delay, stops, queue lengths, fuel consumption, pollution, and
operating costs.


**REQUIRED DATA AND SOURCES**

The analysis of a basic freeway or multilane highway segment requires
details concerning the geometric characteristics of the segment and the
demand characteristics of the users of the segment. Exhibit 12-18 shows the
data that are required to conduct an operational analysis and suggested
default values when site-specific data are unavailable ( _18_ ). The analyst may
replace the default values of Exhibit 12-18 with defaults that have been
locally calibrated.

The exhibit further distinguishes between urban and rural conditions for
certain defaults. The classification of a facility into urban and rural is made
on the basis of the FHWA smoothed or adjusted urbanized boundary
definition ( _8_ ), which in turn is derived from Census data.


**Exhibit 12-18: Required Input Data, Potential Data Sources, and Default Values for**
**Basic Freeway and Multilane Highway Segment Automobile Analysis**


**Required Data and Units** **Potential Data Source(s)** **Suggested Default Value**

_Geometric Data—Basic Freeway Segments_

Direct speed measurements, Base free-flow speed: speed
**Free-flow speed** (mi/h) estimate from design speed limit + 5 mi/h (range 55–75
or speed limit mi/h)

_**Number of mainline**_
_**freeway lanes in one**_ Road inventory, aerial photo At least 2
_**direction**_ (ln)

Lane width (ft) Road inventory, aerial photo 12 ft (range 10–12 ft)
Right-side lateral clearance
Road inventory, aerial photo 10 ft (range 0–10 ft)
(ft)

Total ramp density Must be provided (range 0–6
Road inventory, aerial photo
(ramps/mi) ramps/mi)
Terrain type (level, rolling, Design plans, analyst
Must be provided
specific grade) judgment

_Geometric Data—Multilane Highway Segments_
**Free-flow speed** (mi/h) Direct speed measurements, Base free-flow speed:


estimate from design speed Speed limit + 5 mi/h (range 50–
or speed limit 70 mi/h)
Speed limit + 7 mi/h (range <50
mi/h)
_**Number of mainline**_
_**freeway lanes (one**_ Road inventory, aerial photo At least 2
_**direction)**_

Lane width (ft) Road inventory, aerial photo 12 ft (range 10–12 ft)
Right-side lateral clearance
Road inventory, aerial photo 6 ft (range 0–6 ft)
(ft)

Median (left-side) lateral
Road inventory, aerial photo 6 ft (range 0–6 ft)
clearance (ft)

8 access points/mi (rural)
16 access points/mi (low
Access point density
Road inventory, aerial photo density suburban)
(points/mi)

25 access points/mi (highdensity suburban)
Terrain type (level, rolling, Design plans, analyst
Must be provided
specific grade) judgment

Median type (divided,
Road inventory, aerial photo Must be provided
undivided, TWLTL)

_Demand Data—Basic Freeway and Multilane Highway Segments_
Hourly demand volume
Field data, modeling Must be provided
(veh/h)

Heavy vehicle percentage 5% (urban)
Field data
(%) 12% (rural) _[a]_

Basic freeway segments 0.94

Peak hour factor _[b]_

Field data Multilane highways 0.95 (urban)
(decimal)
or 0.88 (rural)
Driver population, capacity,
and free-flow speed Field data 1.0 (see also Chapter 26)
adjustment factors


Notes: _**Bold italic**_ indicates high sensitivity (>20% change) of service measure to the choice
of default value.
**Bold** indicates moderate sensitivity (10%–20% change) of service measure to the
choice of default value.
TWLTL = two-way left-turn lane.
_a_ See Chapter 26 in Volume 4 for state-specific default heavy vehicle percentages
and driver population adjustment factors.
_b_ Moderate to high sensitivity of service measures for very low PHF values. See the
discussion in the text. PHF is not required when peak 15-min demand volumes are
provided.


Research into the percentage of heavy vehicles on uninterrupted-flow
facilities ( _18_ ) found a wide range of values from state to state. Section 2 of
Chapter 26 provides state-specific defaults for heavy vehicle percentage on
the basis of data from the 2004 Highway Performance Monitoring System.
States or local jurisdictions that have developed their own values may
substitute them. Analysts may wish to develop their own default values on the
basis of recent data.


**OVERVIEW OF THE METHODOLOGY**

Exhibit 12-19 illustrates the basic methodology used in operational
analysis. The methodology can also be directly applied to determine the
number of lanes required to provide a target LOS for a given demand
volume.


**Exhibit 12-19: Overview of Operational Analysis Methodology for Basic Freeway and**
**Multilane Highway Segments**


**COMPUTATIONAL STEPS**


**Step 1: Input Data**

For a typical operational analysis, as noted previously, the analyst would
have to specify (with either site-specific or default values) the demand
volume; number and width of lanes; right-side or overall lateral clearance;
total ramp or access point density; percent of heavy vehicles; PHF; terrain;
and the driver population, speed, and capacity adjustment factors (if
necessary).


**Step 2: Estimate and Adjust FFS**

FFS can be determined directly from field measurements or can be
estimated as described below. Statement of FFS in 5-mi/h increments is no
longer necessary. This change is important in accounting for the effect of
weather or work zones, which may reduce the value of the base FFS.


_Field Measurement of FFS_

FFS is the mean speed of passenger cars measured during periods of low
to moderate flow (up to 500 pc/h/ln). For a specific freeway or multilane
highway segment, average speeds are virtually constant in this range of flow
rates. Field measurement of FFS, if possible, is preferable. If the FFS is
measured directly, no adjustments are applied to the measured value.

Some freeways may have lower posted speed limits for trucks, which
may affect the mixed-flow FFS. In these cases, field studies are
recommended, since the FFS estimation methodology below is not sensitive
to the posted speed limit or the presence of a high percentage of trucks.

The speed study should be conducted at a location that is representative
of the segment at a time when flow rates are less than 1,000 pc/h/ln. The
speed study should measure the speeds of all passenger cars or use a
systematic sample (e.g., every 10th car in each lane). A sample of at least
100 passenger car speeds should be obtained. Any speed measurement
technique that has been found acceptable for other types of traffic engineering
applications may be used. Further guidance on the conduct of speed studies is
provided in standard traffic engineering publications, such as the _Manual of_
_Transportation Engineering Studies_ ( _16_ ).


_Estimating FFS_


_Basic Freeway Segments_

Field measurements for future facilities are not possible, and field
measurement may not be possible or practical for all existing facilities. In
such cases, the segment’s FFS may be estimated by using Equation 12-2,
which is based on the physical characteristics of the segment under study:


**Equation 12-2:**


where

_FFS_ = free-flow speed of the basic freeway segment (mi/h);

_BFFS_ = base FFS for the basic freeway segment (mi/h);

_fLW_ = adjustment for lane width, from Exhibit 12-20 (mi/h);

_fRLC_ = adjustment for right-side lateral clearance, from Exhibit 12-21
(mi/h); and

_TRD_ = total ramp density (ramps/mi).


_Multilane Highway Segments_

For multilane highway segments, the FFS can be estimated by using
Equation 12-3, which is based on the physical characteristics of the segment
under study. It is evident that while the base FFS and the lane width
adjustment are shared with the estimation method for basic freeway segments
in Equation 12-2, the remaining terms are unique to multilane highway
segments:


**Equation 12-3:**


where

_FFS_ = free-flow speed of the multilane highway segment (mi/h);


_BFFS_ = base FFS for the multilane highway segment (mi/h);

_fLW_ = adjustment for lane width, from Exhibit 12-20 (mi/h);

_fTLC_ = adjustment for total lateral clearance, from Exhibit 12-22
(mi/h);

_fM_ = adjustment for median type, from Exhibit 12-23 (mi/h); and

_fA_ = adjustment for access point density, from Exhibit 12-24 (mi/h).


_Adjustments to FFS_


_Base FFS_

This methodology covers basic freeway segments with a FFS in the range
of 55 to 75 mi/h. The predictive algorithm for FFS therefore starts with a
value greater than 75 mi/h, specifically a default base FFS of 75.4 mi/h,
which resulted in the most accurate predictions in the underlying research.

The methodology covers multilane highway segments with a FFS in the
range of 45 to 70 mi/h. The most significant value in Equation 12-3 is _BFFS_ .
There is not a great deal of information available to help establish a base
value. In one sense, it is like the design speed—it represents the potential
FFS based only on the highway’s horizontal and vertical alignment, not
including the impacts of lane widths, lateral clearances, median type, and
access points. The design speed may be used for _BFFS_ if it is available.

Although speed limits are not always uniformly set, _BFFS_ for multilane
highways may be estimated, if necessary, as the posted or statutory speed
limit plus 5 mi/h for speed limits 50 mi/h and higher and as the speed limit
plus 7 mi/h for speed limits less than 50 mi/h.


_Adjustment for Lane Width_

The base condition for lane width is 12 ft or greater. When the average
lane width across all lanes is less than 12 ft, the FFS is negatively affected.
Adjustments to reflect the effect of narrower average lane width are shown in
Exhibit 12-20.


**Exhibit 12-20: Adjustment to FFS for Average Lane Width for Basic Freeway and**
**Multilane Highway Segments**


**Average Lane Width (ft)** **Reduction in FFS,** _**fLW**_ **(mi/h)**



≥12
≥11–12
≥10–11



0.0
1.9
6.6



_Adjustment for Right Lateral Clearance on Freeway Segments_

The base condition for right-side lateral clearance is 6 ft or greater. The
lateral clearance is measured from the right edge of the travel lane to the
nearest lateral obstruction. Care must be taken in identifying a “lateral
obstruction.” Some obstructions may be continuous, such as retaining walls,
concrete barriers, guardrails, or barrier curbs. Others may be periodic, such
as light supports or bridge abutments. In some cases, drivers may become
accustomed to certain types of obstructions, and their influence on traffic is
often negligible.

Exhibit 12-21 shows the adjustment to FFS due to the existence of
obstructions closer than 6 ft from the right travel lane edge. Median
clearances of 2 ft or more on the left side of the travel lanes generally have
little impact on traffic. No adjustments are available to reflect the presence
of left-side lateral obstructions closer than 2 ft from the left travel lane edge.
Such situations are rare on modern freeways, except in constrained work
zones.


**Exhibit 12-21: Adjustment to FFS for Right-Side Lateral Clearance,** _**fRLC**_ **(mi/h), for**
**Basic Freeway Segments**














|Right-Side Lateral Clearance (ft)|Lanes in One Direction<br>2 3 4 ≥5|
|---|---|
|≥6<br>5<br>4<br>3<br>2<br>1<br>0|0.0<br>0.6<br>1.2<br>1.8<br>2.4<br>3.0<br>3.6<br>0.0<br>0.4<br>0.8<br>1.2<br>1.6<br>2.0<br>2.4<br>0.0<br>0.2<br>0.4<br>0.6<br>0.8<br>1.0<br>1.2<br>0.0<br>0.1<br>0.2<br>0.3<br>0.4<br>0.5<br>0.6|



Note: Interpolate for noninteger values of right-side lateral clearance.


The impact of a right-side lateral clearance restriction depends on both
the distance to the obstruction and the number of lanes in one direction on the
basic freeway segment. A lateral clearance restriction causes vehicles in the
right lane to move somewhat to the left. This movement, in turn, affects
vehicles in the next lane. As the number of lanes increases, the overall effect
on freeway operations decreases.


_Adjustment for Total Lateral Clearance on Multilane Highway_
_Segments_

The adjustment for total lateral clearance (TLC) on multilane highway
segments is based on TLC at the roadside (right side) and at the median (left
side). Fixed obstructions with lateral clearance effects include light
standards, signs, trees, abutments, bridge rails, traffic barriers, and retaining
walls. Standard raised curbs are not considered to be obstructions.

Right-side lateral clearance is measured from the right edge of the travel
lanes to the nearest periodic or continuous roadside obstruction. If such
obstructions are farther than 6 ft from the edge of the pavement, a value of 6
ft is used.

Left-side lateral clearance is measured from the left edge of the travel
lanes to the nearest periodic or continuous obstruction in the median. If such
obstructions are farther than 6 ft from the edge of the pavement, a value of 6
ft is used.

Left-side lateral clearances are subject to some judgment. Many types of
common median barriers do not affect driver behavior if they are no closer
than 2 ft from the edge of the travel lane, including concrete and W-beam
barriers. A value of 6 ft would be used in such cases. Also, when the
multilane highway segment is undivided or has a TWLTL, no left-side lateral
clearance restriction is assumed, and a value of 6 ft is applied. A separate
adjustment, described next, accounts for the impact of an undivided highway
on FFS.

Equation 12-4 is used to determine TLC:


**Equation 12-4:**


where

_TLC_ = total lateral clearance (ft) (maximum value 12 ft),

_LCR_ = right-side lateral clearance (ft) (maximum value 6 ft), and

_LCL_ = left-side lateral clearance (ft) (maximum value 6 ft).


Exhibit 12-22 shows the reduction in FFS due to lateral obstructions on
the multilane highway.


**Exhibit 12-22: Adjustment to FFS for Lateral Clearances for Multilane Highways**










|Four-Lane Highways<br>TLC (ft) Reduction in FFS, f (mi/h)<br>TLC|Six-Lane Highways<br>TLC (ft) Reduction in FFS, f (mi/h)<br>TLC|
|---|---|
|12<br>10<br>8<br>6<br>4<br>2<br>0<br>0.0<br>0.4<br>0.9<br>1.3<br>1.8<br>3.6<br>5.4|12<br>10<br>8<br>6<br>4<br>2<br>0<br>0.0<br>0.4<br>0.9<br>1.3<br>1.7<br>2.8<br>3.9|



Note: Interpolation to the nearest 0.1 is recommended.


_Adjustment for Type of Median on Multilane Highways_

The adjustment for type of median is given in Exhibit 12-23. Undivided
multilane highways reduce the FFS by 1.6 mi/h.


**Exhibit 12-23: Adjustment to FFS for Median Type for Multilane Highways**


**Median Type** **Reduction in FFS,** _**fM**_ **(mi/h)**

Undivided 1.6
TWLTL 0.0
Divided 0.0


_Adjustment for Total Ramp Density on Basic Freeway Segments_

Equation 12-2 includes a term that accounts for the impact of total ramp
density on FFS. Total ramp density is defined as the number of ramps (on and
off, one direction) located between 3 mi upstream and 3 mi downstream of


the midpoint of the basic freeway segment under study, divided by 6 mi. The
total ramp density has been found to be a measure of the impact of merging
and diverging vehicles on FFS.


_Adjustment for Access Point Density on Multilane Highway_
_Segments_

Exhibit 12-24 presents the adjustment to FFS for various levels of access
point density. Studies indicate that for each access point per mile, the
estimated FFS decreases by approximately 0.25 mi/h, regardless of the type
of median.

The number of access points per mile is determined by dividing the total
number of access points (i.e., driveways and unsignalized intersections) on
the right side of the highway in the direction of travel by the length of the
segment in miles. An intersection or driveway should only be included in the
count if it influences traffic flow. Access points that go unnoticed by drivers
or that have little activity should not be used to determine access point
density.


**Exhibit 12-24: Adjustment to FFS for Access Point Density for Multilane Highways**


**Access Point Density (access points/mi)** **Reduction in FFS,** _**fA**_ **(mi/h)**

0 0.0
10 2.5
20 5.0
30 7.5
≥40 10.0


Note: Interpolation to the nearest 0.1 is recommended.


Although the calibration of this adjustment did not include one-way
multilane highway segments, inclusion of intersection approaches and
driveways on both sides of the facility might be appropriate in determining
the access point density on one-way segments.


_Speed Adjustment Factor for Basic Freeway Segments_

The estimated FFS for basic freeway segments can be further adjusted to
reflect, for example, effects of inclement weather. In this case, an adjusted


free-flow speed _FFSadj_ is calculated by multiplying the FFS by a SAF as
shown in Equation 12-5:


**Equation 12-5:**


where _SAF_ is the speed adjustment factor. The speed adjustment factor can
represent a combination of sources, including weather and work zone effects.
Default speed adjustment factors and guidance for how to apply them are
found in Chapter 11.

The SAF may also be used to calibrate the estimated FFS for local
conditions or other effects that contribute to a reduction in FFS. For example,
poor pavement conditions or sun glare may cause drivers to reduce their
speeds even under low-volume conditions. The adjusted FFS can be used
directly in the speed–flow relationship for basic freeway segments in Exhibit
12-6 to define a continuous speed–flow curve that explicitly considers this
adjusted FFS. Finally, the effect of unfamiliar drivers on FFS can also be
accounted for by using an adjusted FFS. While the driver population SAF
defaults to 1.0 in the base procedure, general guidance for selecting an
appropriate SAF to account for this factor is given in Section 4 of Chapter
26.

No adjustment of the speed–flow equation using these SAFs is possible
for multilane highway segments, since no empirical research exists for
applying these effects on multilane highways.


**Step 3: Estimate and Adjust Capacity**

In this step, the base capacity for the basic freeway or multilane highway
segment is estimated. The segment capacity is principally a function of the
segment FFS, but it can be adjusted to calibrate the segment for local
conditions or to reflect impacts of adverse weather conditions, incidents, or
other factors. The base capacity values for basic freeway segments and
multilane highway segments are listed in Exhibit 12-4 for various values of
FFS. Because of the ability to interpolate between different FFS values, the
resulting segment capacities should also be interpolated. Alternatively, the


base capacity _c_ for a basic freeway segment (in passenger cars per hour per
lane) can be estimated directly with Equation 12-6, while the base capacity
for a multilane highway segment can be estimated directly with Equation 127:


**Equation 12-6:**


**Equation 12-7:**


where all variables have been previously defined.

The capacities resulting from application of these equations can never
exceed the base capacities listed in Exhibit 12-4, which are 2,400 pc/h/ln for
basic freeway segments and 2,300 pc/h/ln for multilane highway segments.
Similarly, the FFS used in these equations should not exceed 75 mi/h for
basic freeway segments or 70 mi/h for multilane highway segments.


_Adjustment to Capacity for Local Calibration_

The base capacities estimated by using Equation 12-6 and Equation 12-7
are based on ideal conditions and are expressed in units of passenger cars
per hour per lane. The presence of a significant proportion of heavy vehicles,
especially in combination with grades, will result in a net decrease in the
observed capacity when converted to units of vehicles per hour per lane. As
a result, sensor-based measurements of freeway capacities (in vehicles per
hour per lane) may be significantly less than the base values stated above.

Many factors other than heavy vehicle effects can contribute to a
reduction in basic freeway segment capacity. Examples of capacity-reducing
effects include the following:

   - Capacity adjustment for driver population, which is intended to
account for the level of unfamiliar drivers in the traffic stream (see
Section 4 of Chapter 26 for additional details);


   - Turbulence generated from lane drops between two basic segments;

   - Turbulence due to merging, diverging, or weaving maneuvers
between two basic segments;

   - Capacity reductions due to poor sight distance—for example, due to
crest vertical curves or horizontal curves;

   - Narrow lane widths or low lateral clearances in addition to the
effects on FFS presented in Step 2;

   - Travel through tunnels or across bridges;

   - Poor pavement conditions; and

   - Friction effects due to roadside features and attractions that cause
drivers to increase following headways.

In these cases, development of a local estimate of capacity and use of that
estimate to calibrate a CAF for the segment under study are highly
recommended. In the absence of generalized national data on these capacityreducing effects, a local calibration study or expert judgment is needed to
produce a reasonable estimate of segment performance. A methodology for
estimating freeway capacities from sensor data is provided in Section 5 of
Chapter 26.


_Adjustment to Capacity for Basic Freeway Segments_

The capacity of a basic freeway segment may be adjusted further to
account for the impacts of adverse weather, driver population, occurrence of
traffic incidents, or a combination of such influences. The methodology for
making these adjustments is the same as that for other types of freeway
segments. CAF defaults are found in Chapter 11, along with additional
discussion on how to apply them. For convenience, a brief summary is
provided here.

The capacity of a basic freeway segment can be adjusted as shown in
Equation 12-8:


**Equation 12-8:**


where

_cadj_ = adjusted capacity of segment (pc/h),

_c_ = base capacity of segment (pc/h), and

_CAF_ = capacity adjustment factor (unitless).


The CAF can have several components, including weather, incident,
work zone, driver population, and calibration adjustments. The adjustments
for weather and incidents are most commonly applied in the context of a
reliability analysis as described in Chapter 11, Freeway Reliability
Analysis. If desired, capacity can be adjusted further to account for
unfamiliar drivers in the traffic stream. While the default CAF for this effect
is set to 1.0, guidance is provided in Section 4 of Chapter 26, where
estimates for the CAF based on the composition of the driver population are
provided.

No adjustment of the speed–flow equation using these CAFs is possible
for multilane highway segments, since no empirical research exists for
applying these effects to multilane highways.


**Step 4: Adjust Demand Volume**

Since the speed–flow curves and parameters of Exhibit 12-6 are based
on flow rates in equivalent passenger cars per hour on the basic freeway
segment, demand volumes expressed as vehicles per hour under prevailing
conditions must be converted to this basis by using Equation 12-9:


**Equation 12-9:**


where

_vp_ = demand flow rate under equivalent base conditions (pc/h/ln),

_V_ = demand volume under prevailing conditions (veh/h),


_PHF_ = peak hour factor (decimal),

_N_ = number of lanes in analysis direction (ln), and

_fHV_ = adjustment factor for presence of heavy vehicles (decimal).


_Peak Hour Factor_

The PHF represents the variation in traffic flow within an hour.
Observations of traffic flow consistently indicate that the flow rates found in
the peak 15 min within an hour are not sustained throughout the entire hour.
The application of the PHF in Equation 12-9 accounts for this phenomenon.

On freeways, typical PHFs range from 0.85 to 0.98 ( _18_ ). On multilane
highways, typical PHFs range from 0.75 to 0.95. Lower values within that
range are typical of lower-volume conditions. Higher values within that
range are typical of urban and suburban peak-hour conditions. Field data
should be used if possible to develop PHFs that represent local conditions.


_Adjustment for Heavy Vehicles_

All heavy vehicles are classified as SUTs or TTs. Recreational vehicles
and buses are treated as SUTs. The heavy vehicle adjustment factor _fHV_ is
computed from the combination of the two heavy vehicle classes, which are
added to get an overall truck percentage _PT_ .


**Equation 12-10:**


where

_fHV_ = heavy vehicle adjustment factor (decimal),

_PT_ = proportion of SUTs and TTs in traffic stream (decimal), and

_ET_ = passenger car equivalent of one heavy vehicle in the traffic
stream (PCEs).


The adjustment factor is found in a two-step process. First, the PCE for
each truck is found for the prevailing conditions under study. These
equivalency values represent the number of passenger cars that would use the
same amount of freeway capacity as one truck under the prevailing
conditions. Second, Equation 12-10 is used to convert the PCE values to the
adjustment factor.

The effect of heavy vehicles on traffic flow depends on the terrain and
grade conditions on the segment as well as traffic composition. PCEs can be
selected for one of two conditions:

   - Extended freeway and multilane highway segments in general terrain,
or

   - Specific upgrades or downgrades.

Each of these conditions is more precisely defined and discussed below.
However, research has shown that PCEs should be used mostly for
addressing capacity and LOS issues. They provide reasonable results for
speeds and densities when the grade is slight or the truck percentage is low.
For combinations that include steep grades, high truck percentages, or both,
the mixed-flow model described in Chapter 25 (for composite grades) and
Chapter 26 (for single grades) is recommended for computing mixed-flow
speeds and densities and automobile and truck speeds in the mixed traffic
stream.


_Equivalents for General Terrain Segments_

General terrain refers to extended lengths of freeway and multilane
highways containing a number of upgrades and downgrades where no one
grade is long enough or steep enough to have a significant impact on the
operation of the overall segment. General terrain can be either level or
rolling. To determine which of these terrain types applies, each upgrade and
downgrade should be considered to be a single grade, even if the grade is not
uniform. The total length of the upgrade or downgrade is used with the
steepest grade it contains. The categorization of a segment as having either
level or rolling terrain is as follows:

   - _Level terrain_ : Any combination of grades and horizontal or vertical
alignment that permits heavy vehicles to maintain the same speed as


passenger cars. This type of terrain typically contains short grades of
no more than 2%.

   - _Rolling terrain_ : Any combination of grades and horizontal or vertical
alignment that causes heavy vehicles to reduce their speed below
those of passenger cars but that does not cause heavy vehicles to
operate at crawl speeds for any significant length.

No PCE is provided for mountainous terrain, which is any combination
of grades and horizontal and vertical alignment that causes heavy vehicles to
operate at crawl speed for significant distances or at frequent intervals. In
this case, the mixed-flow model presented in Chapters 25 and 26 must be
used to estimate speeds and densities. Exhibit 12-25 gives PCEs for the
default mix of trucks under level and rolling terrain conditions.


**Exhibit 12-25: PCEs for General Terrain Segments**







|Passenger Car Equivalent|Terrain Type<br>Level Rolling|
|---|---|
|_ET_|2.0<br>3.0|


_Equivalents for Specific Upgrades_

Freeway and multilane highway segments longer than 0.5 mi with grades
between 2% and 3% or longer than 0.25 mi with grades of 3% or greater
should be considered as separate segments. Research ( _19_ ) has revealed that
the SUT population on freeways has a median weight-to-horsepower ratio of
about 100 lb/hp while the TT population has a median weight-to-horsepower
ratio of 150 lb/hp. These values can vary from one setting to another.

Exhibit 12-26 gives specific-segment PCE values for a 30%/70%
SUT/TT mix, Exhibit 12-27 gives PCE values for a 50%/50% mix, and
Exhibit 12-28 gives PCE values for a 70%/30% mix. The 30% SUT
condition occurs more frequently on rural facilities; the 50% condition
occurs more frequently on urban facilities. Exhibit 12-28 is recommended for
conditions where the majority of the trucks in the traffic stream are SUTs.
Note that for the exhibits, segment lengths for grades above 3.5% are limited
to 1 mi, because steeper grades are rarely longer than this in practice.


**Exhibit 12-26: PCEs for a Mix of 30% SUTs and 70% TTs**


|% Grade Length (mi)|Percentage of Trucks (%)|
|---|---|
|**% Grade**<br>**Length (mi)**|**2%**<br>**4%**<br>**5%**<br>**6%**<br>**8%**<br>**10%**<br>**15%**<br>**20%**<br>**>25%**|
|-2<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97|
|0<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97|
|2<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>3.76<br>2.96<br>2.78<br>2.65<br>2.48<br>2.38<br>2.22<br>2.14<br>2.09<br>4.47<br>3.33<br>3.08<br>2.91<br>2.68<br>2.54<br>2.34<br>2.23<br>2.17<br>4.80<br>3.50<br>3.22<br>3.03<br>2.77<br>2.61<br>2.39<br>2.28<br>2.21<br>5.00<br>3.60<br>3.30<br>3.09<br>2.83<br>2.66<br>2.42<br>2.30<br>2.23<br>5.04<br>3.62<br>3.32<br>3.11<br>2.84<br>2.67<br>2.43<br>2.31<br>2.23|
|2.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>4.11<br>3.14<br>2.93<br>2.78<br>2.58<br>2.46<br>2.28<br>2.19<br>2.13<br>5.04<br>3.62<br>3.32<br>3.11<br>2.84<br>2.67<br>2.43<br>2.31<br>2.23<br>5.48<br>3.85<br>3.51<br>3.27<br>2.96<br>2.77<br>2.50<br>2.36<br>2.28<br>5.73<br>3.98<br>3.61<br>3.36<br>3.03<br>2.83<br>2.54<br>2.40<br>2.31<br>5.80<br>4.02<br>3.64<br>3.38<br>3.05<br>2.84<br>2.55<br>2.41<br>2.32|
|3.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>4.88<br>3.54<br>3.25<br>3.05<br>2.80<br>2.63<br>2.41<br>2.29<br>2.22<br>6.34<br>4.30<br>3.87<br>3.58<br>3.20<br>2.97<br>2.64<br>2.48<br>2.38<br>7.03<br>4.66<br>4.16<br>3.83<br>3.39<br>3.12<br>2.76<br>2.57<br>2.46<br>7.44<br>4.87<br>4.33<br>3.97<br>3.50<br>3.22<br>2.82<br>2.62<br>2.50<br>7.53<br>4.92<br>4.38<br>4.01<br>3.53<br>3.24<br>2.84<br>2.63<br>2.51|
|4.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1|2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>5.80<br>4.02<br>3.64<br>3.38<br>3.05<br>2.84<br>2.55<br>2.41<br>2.32<br>7.90<br>5.11<br>4.53<br>4.14<br>3.63<br>3.32<br>2.90<br>2.68<br>2.55<br>8.91<br>5.64<br>4.96<br>4.50<br>3.92<br>3.56<br>3.07<br>2.82<br>2.67<br>9.19<br>5.78<br>5.08<br>4.60<br>3.99<br>3.62<br>3.11<br>2.85<br>2.70|
|5.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1|2.62<br>2.37<br>2.30<br>2.24<br>2.17<br>2.12<br>2.04<br>1.99<br>1.97<br>6.87<br>4.58<br>4.10<br>3.77<br>3.35<br>3.09<br>2.73<br>2.55<br>2.44<br>9.78<br>6.09<br>5.33<br>4.82<br>4.16<br>3.76<br>3.21<br>2.93<br>2.77<br>11.20<br>6.83<br>5.94<br>5.33<br>4.56<br>4.09<br>3.45<br>3.12<br>2.93<br>11.60<br>7.04<br>6.11<br>5.47<br>4.67<br>4.18<br>3.51<br>3.17<br>2.97|
|||


Note: Interpolation in the exhibit is permitted.


**Exhibit 12-27: PCEs for a Mix of 50% SUTs and 50% TTs**








|% Grade Length (mi)|Percentage of Trucks (%)<br>2% 4% 5% 6% 8% 10% 15% 20% >25%|
|---|---|
|-2<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93|
|0<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93|
|2<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>3.76<br>2.95<br>2.77<br>2.64<br>2.47<br>2.36<br>2.20<br>2.11<br>2.06<br>4.32<br>3.24<br>3.01<br>2.84<br>2.63<br>2.49<br>2.29<br>2.19<br>2.12<br>4.57<br>3.37<br>3.11<br>2.93<br>2.70<br>2.55<br>2.33<br>2.22<br>2.15<br>4.71<br>3.45<br>3.17<br>2.99<br>2.74<br>2.58<br>2.36<br>2.24<br>2.17<br>4.74<br>3.47<br>3.19<br>3.00<br>2.75<br>2.59<br>2.36<br>2.24<br>2.17|
|2.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>4.10<br>3.13<br>2.92<br>2.77<br>2.57<br>2.44<br>2.26<br>2.16<br>2.10<br>4.84<br>3.52<br>3.23<br>3.03<br>2.77<br>2.61<br>2.38<br>2.26<br>2.18<br>5.17<br>3.69<br>3.37<br>3.15<br>2.87<br>2.69<br>2.43<br>2.30<br>2.22<br>5.36<br>3.79<br>3.45<br>3.22<br>2.92<br>2.73<br>2.47<br>2.33<br>2.24<br>5.40<br>3.81<br>3.47<br>3.24<br>2.93<br>2.74<br>2.47<br>2.33<br>2.25|
|3.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>4.89<br>3.54<br>3.25<br>3.05<br>2.79<br>2.62<br>2.39<br>2.26<br>2.19<br>6.05<br>4.15<br>3.75<br>3.47<br>3.11<br>2.89<br>2.58<br>2.42<br>2.32<br>6.58<br>4.43<br>3.97<br>3.66<br>3.26<br>3.01<br>2.67<br>2.49<br>2.39<br>6.88<br>4.58<br>4.10<br>3.77<br>3.35<br>3.09<br>2.72<br>2.53<br>2.42<br>6.95<br>4.62<br>4.13<br>3.80<br>3.37<br>3.10<br>2.73<br>2.54<br>2.43|
|4.5<br>0.125|2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93|


|0.375<br>0.625<br>0.875<br>1|5.83 4.03 3.65 3.39 3.05 2.84 2.55 2.39 2.30<br>7.53 4.92 4.38 4.01 3.53 3.24 2.83 2.62 2.50<br>8.32 5.34 4.72 4.29 3.75 3.42 2.97 2.73 2.59<br>8.53 5.45 4.81 4.37 3.81 3.47 3.00 2.76 2.62|
|---|---|
|5.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1|2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>6.97<br>4.63<br>4.14<br>3.81<br>3.38<br>3.11<br>2.74<br>2.55<br>2.43<br>9.37<br>5.89<br>5.16<br>4.68<br>4.05<br>3.67<br>3.14<br>2.88<br>2.72<br>10.49<br>6.48<br>5.65<br>5.09<br>4.37<br>3.93<br>3.34<br>3.03<br>2.85<br>10.80<br>6.64<br>5.78<br>5.20<br>4.46<br>4.01<br>3.39<br>3.08<br>2.89|
|6<br>0.125<br>0.375<br>0.625<br>0.875<br>1|2.67<br>2.38<br>2.31<br>2.25<br>2.16<br>2.11<br>2.02<br>1.97<br>1.93<br>7.64<br>4.98<br>4.43<br>4.05<br>3.56<br>3.26<br>2.85<br>2.64<br>2.51<br>10.45<br>6.45<br>5.63<br>5.07<br>4.36<br>3.92<br>3.33<br>3.03<br>2.85<br>11.78<br>7.16<br>6.20<br>5.56<br>4.74<br>4.24<br>3.56<br>3.22<br>3.01<br>12.15<br>7.35<br>6.36<br>5.69<br>4.85<br>4.33<br>3.62<br>3.27<br>3.05|



Note: Interpolation in the exhibit is permitted.


**Exhibit 12-28: PCEs for a Mix of 70% SUTs and 30% TTs**












|% Grade Length (mi)|Percentage of Trucks (%)<br>2% 4% 5% 6% 8% 10% 15% 20% >25%<br>2.39 2.18 2.12 2.07 2.01 1.96 1.89 1.85 1.83<br>2.39 2.18 2.12 2.07 2.01 1.96 1.89 1.85 1.83<br>2.39 2.18 2.12 2.07 2.01 1.96 1.89 1.85 1.83<br>2.39 2.18 2.12 2.07 2.01 1.96 1.89 1.85 1.83<br>2.39 2.18 2.12 2.07 2.01 1.96 1.89 1.85 1.83<br>2.39 2.18 2.12 2.07 2.01 1.96 1.89 1.85 1.83|
|---|---|
|-2<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|-2<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|
|0<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.39<br>2.18<br>2.12<br>2.07<br>2.01<br>1.96<br>1.89<br>1.85<br>1.83<br>2.39<br>2.18<br>2.12<br>2.07<br>2.01<br>1.96<br>1.89<br>1.85<br>1.83<br>2.39<br>2.18<br>2.12<br>2.07<br>2.01<br>1.96<br>1.89<br>1.85<br>1.83<br>2.39<br>2.18<br>2.12<br>2.07<br>2.01<br>1.96<br>1.89<br>1.85<br>1.83<br>2.39<br>2.18<br>2.12<br>2.07<br>2.01<br>1.96<br>1.89<br>1.85<br>1.83<br>2.39<br>2.18<br>2.12<br>2.07<br>2.01<br>1.96<br>1.89<br>1.85<br>1.83|
|2<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.67<br>2.32<br>2.23<br>2.17<br>2.08<br>2.03<br>1.94<br>1.89<br>1.86<br>3.63<br>2.82<br>2.64<br>2.52<br>2.35<br>2.25<br>2.10<br>2.02<br>1.97<br>4.12<br>3.08<br>2.85<br>2.69<br>2.49<br>2.36<br>2.18<br>2.08<br>2.02<br>4.37<br>3.21<br>2.96<br>2.78<br>2.56<br>2.42<br>2.22<br>2.11<br>2.05<br>4.53<br>3.29<br>3.02<br>2.84<br>2.60<br>2.45<br>2.24<br>2.13<br>2.07<br>4.58<br>3.31<br>3.04<br>2.86<br>2.61<br>2.46<br>2.25<br>2.14<br>2.07|
|2.5<br>0.125<br>0.375<br>0.625<br>0.875|2.75<br>2.36<br>2.27<br>2.20<br>2.11<br>2.04<br>1.95<br>1.90<br>1.87<br>4.01<br>3.02<br>2.80<br>2.65<br>2.46<br>2.33<br>2.16<br>2.06<br>2.01<br>4.66<br>3.35<br>3.08<br>2.88<br>2.64<br>2.48<br>2.26<br>2.15<br>2.08<br>4.99<br>3.52<br>3.21<br>3.00<br>2.73<br>2.56<br>2.32<br>2.19<br>2.12|


|1.25<br>1.5|5.20 3.64 3.30 3.08 2.79 2.60 2.35 2.22 2.14<br>5.26 3.67 3.33 3.10 2.80 2.62 2.36 2.23 2.15|
|---|---|
|3.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1.25<br>1.5|2.93<br>2.45<br>2.34<br>2.26<br>2.16<br>2.09<br>1.98<br>1.92<br>1.89<br>4.86<br>3.46<br>3.16<br>2.96<br>2.69<br>2.53<br>2.30<br>2.18<br>2.10<br>5.88<br>3.99<br>3.59<br>3.32<br>2.98<br>2.76<br>2.46<br>2.31<br>2.22<br>6.40<br>4.26<br>3.81<br>3.51<br>3.12<br>2.88<br>2.55<br>2.38<br>2.28<br>6.74<br>4.43<br>3.96<br>3.63<br>3.21<br>2.96<br>2.60<br>2.42<br>2.32<br>6.83<br>4.48<br>3.99<br>3.66<br>3.24<br>2.98<br>2.62<br>2.44<br>2.33|
|4.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1|3.13<br>2.56<br>2.43<br>2.34<br>2.21<br>2.13<br>2.01<br>1.95<br>1.91<br>5.88<br>3.99<br>3.59<br>3.32<br>2.98<br>2.76<br>2.46<br>2.31<br>2.22<br>7.35<br>4.75<br>4.22<br>3.85<br>3.39<br>3.10<br>2.71<br>2.51<br>2.39<br>8.11<br>5.15<br>4.54<br>4.13<br>3.60<br>3.27<br>2.83<br>2.61<br>2.47<br>8.33<br>5.27<br>4.63<br>4.21<br>3.66<br>3.33<br>2.87<br>2.64<br>2.50|
|5.5<br>0.125<br>0.375<br>0.625<br>0.875<br>1|3.37<br>2.69<br>2.53<br>2.42<br>2.28<br>2.19<br>2.05<br>1.98<br>1.94<br>7.09<br>4.62<br>4.11<br>3.76<br>3.31<br>3.04<br>2.66<br>2.47<br>2.36<br>9.13<br>5.68<br>4.97<br>4.49<br>3.88<br>3.51<br>3.00<br>2.74<br>2.59<br>10.21<br>6.24<br>5.43<br>4.88<br>4.18<br>3.76<br>3.18<br>2.89<br>2.71<br>10.52<br>6.41<br>5.57<br>5.00<br>4.27<br>3.83<br>3.24<br>2.93<br>2.75|
|6<br>0.125<br>0.375<br>0.625<br>0.875<br>1|3.51<br>2.76<br>2.59<br>2.47<br>2.32<br>2.22<br>2.08<br>2.00<br>1.95<br>7.78<br>4.98<br>4.40<br>4.01<br>3.51<br>3.20<br>2.78<br>2.56<br>2.44<br>10.17<br>6.23<br>5.42<br>4.87<br>4.17<br>3.75<br>3.18<br>2.88<br>2.71<br>11.43<br>6.88<br>5.95<br>5.32<br>4.53<br>4.04<br>3.39<br>3.06<br>2.86<br>11.81<br>7.08<br>6.11<br>5.46<br>4.64<br>4.13<br>3.45<br>3.11<br>2.90|



Note: Interpolation in the exhibit is permitted.


The PCE values shown in this chapter have been estimated from
simulation. They are also based on generalized analytical equations for the
propulsion and resistance characteristics of SUTs and TTs ( _19_ ). Different
models based on more detailed vehicle dynamics simulators (e.g., _20_, _21_ )
can produce different results. The PCEs establish an equivalency between
the mixed-traffic capacity and the automobile-only capacity. The speeds
associated with these PCE values are space mean speeds, and the densities
are defined over the length of the segment. As noted previously, in evaluating
composite grades, steep single grades, very high truck percentages, or a
combination, the appropriate mixed-flow model from Chapter 25 (composite
grades) or Chapter 26 (single grades) is recommended in lieu of applying
PCEs.


_Check for LOS F_


At this point, the demand volume has been converted to a demand flow
rate in passenger cars per hour per lane under equivalent base conditions.
This demand rate must be compared with the base capacity of the basic
freeway or multilane highway segment (see Exhibit 12-4).

If demand exceeds capacity, the LOS is F and a breakdown has been
identified. To analyze the impacts of such a breakdown, the Chapter 10
methodology must be used. No further analysis using the present chapter’s
methodology is possible. If demand is less than or equal to capacity, the
analysis continues to Step 5.


**Step 5: Estimate Speed and Density**

At this point in the methodology, the following have been determined: ( _a_ )
the FFS and appropriate FFS curve for use in the analysis and ( _b_ ) the demand
flow rate expressed in passenger cars per hour per lane under equivalent
base conditions. With this information, the speed and density of the traffic
stream may be estimated.

With the equations specified in Exhibit 12-6, the expected mean speed of
the traffic stream can be computed. A graphical solution with Exhibit 12-7
can also be performed.

After the speed is estimated, Equation 12-11 is used to estimate the
density of the traffic stream:


**Equation 12-11:**


where

_D_ = density (pc/mi/ln),

_vp_ = demand flow rate (pc/h/ln), and

_S_ = mean speed of traffic stream under base conditions (mi/h).


As has been noted, Equation 12-11 is only used when _vp/c_ is less than or
equal to 1.00. All cases in which this ratio is greater than 1.00 are LOS F. In
these cases, the speed _S_ will be outside the range of Exhibit 12-6 and Exhibit
12-7, and no speed can be estimated.

Where LOS F exists, the analyst should consult Chapter 10, which allows
an analysis of the time and spatial impacts of a breakdown, including its
effects on upstream and downstream segments.


**Step 6: Determine LOS**

Exhibit 12-15 is entered with the density obtained from Equation 12-11
to determine the expected prevailing LOS.


## **4. EXTENSIONS TO THE METHODOLOGY**

**BASIC MANAGED LANE SEGMENTS**

This section provides information specific to managed lanes that can be
used in conjunction with the core motorized vehicle methodology to analyze
the operation of basic managed lane segments on freeways. Section 2,
Concepts, defines the five types of basic managed lane segments and presents
basic speed–flow and capacity concepts for managed lanes.

Operating speeds and capacities of managed lanes are a function of how
the managed lanes are separated from the general purpose lanes, the number
of managed lanes, and, in the case of continuous access and Buffer 1
managed lane segments, operational conditions in the adjacent general
purpose lanes.

The general form of the speed–flow relationship for managed lanes is
illustrated in Exhibit 12-29, where the _x_ -axis represents the adjusted 15-min
demand flow rate _vp_ and the _y_ -axis gives the space mean speed _SML_ for the
traffic stream.

The exhibit distinguishes two speed–flow curves that depend on a
frictional effect between the managed lanes and adjacent general purpose
lane. Managed lanes with continuous access or Buffer 1 separation exhibit a
deteriorated performance as the general purpose lanes approach capacity
(i.e., their density exceeds 35 pc/mi/ln).


**Exhibit 12-29: General Form for Speed–Flow Curves for Basic Managed Lane**
**Segments on Freeways**


The general analytic form of the speed–flow relationship is given by
Equation 12-12, along with the equations for determining the model
parameters including the breakpoint and the capacity, both of which are
based on FFS.


**Equation 12-12:**


where

_SML_ = space mean speed of the basic managed lane segment (mi/h);

_S_ 1 = speed within the linear portion of the speed–flow curve, from
Equation 12-15 (mi/h);

_S_ 2 = speed drop within the curvilinear portion of the speed–flow


curve, from Equation 12-17 (mi/h);

_S_ 3 = additional speed drop (mi/h) within the curvilinear portion of the
speed–flow curve when the density of the adjacent general
purpose lane is more than 35 pc/mi/ln, from Equation 12-19;

_Ic_ = indicator variable, where 1 = presence of densities greater than
35 pc/mi/ln in the adjacent general purpose lane (0 or 1);

_BP_ = breakpoint in the speed–flow curve separating the linear and
curvilinear sections (pc/h/ln), from Equation 12-13; and

_vp_ = 15-min average flow rate (pc/h/ln).


The breakpoint in the speed–flow curve is defined by Equation 12-13:


**Equation 12-13:**


where

_BP_ = breakpoint in the speed–flow curve separating the linear and
curvilinear sections (pc/h/ln);

_BP_ 75 = breakpoint for a FFS of 75 mi/h, from Exhibit 12-30 (pc/h/ln);

_λBP_ = rate of increase in breakpoint per unit decrease in FFS, from
Exhibit 12-30 (pc/h/ln);

_FFSadj_ = adjusted free-flow speed (mi/h); and

_CAF_ = capacity adjustment factor (unitless).


Similar to general purpose lanes, capacity and FFS can be adjusted to
account for the impacts of weather, incidents, and work zones and for overall
calibration purposes. Research specific to managed lanes on the magnitude
of these effects is limited, but the same adjustments provided for basic
segments can be considered. Default CAF and SAF values for basic
segments are provided in Chapter 11. The default values do not explicitly list
single-lane facilities, but in the absence of field data, defaults given for two

lane facilities may be used (e.g., for a single-lane managed lane shoulder
closure incident).

A basic managed lane segment’s capacity is estimated by Equation 1214:


**Equation 12-14:**


where

_cadj_ = adjusted basic managed lane segment capacity (pc/h/ln);

_CAF_ = capacity adjustment factor (unitless);

_c_ 75 = managed lane capacity for a FFS of 75 mi/h, from Exhibit 1230 (pc/h/ln);

_λc_ = rate of change in capacity per unit change in FFS, from
Exhibit 12-30 (pc/h/ln); and

_FFSadj_ = adjusted free-flow speed (mi/h).


The linear portion of the speed–flow curve is computed from Equation
12-15:


**Equation 12-15:**


where _A_ 1 is the speed reduction per unit of flow rate in the linear section of
the speed–flow curve (mi/h), from Exhibit 12-30, and all other variables are
as defined previously.

The curvilinear portion of the speed–flow curve for basic managed lane
segments is characterized by using a calibration factor _A_ 2 that is computed
with Equation 12-16:


**Equation 12-16:**


where

_A_ 2 = speed reduction per unit of flow rate in the curvilinear section
of the speed–flow curve (mi/h);

_A_ 255 = calibration factor for a FFS of 55 mi/h, from Exhibit 12-30
(mi/h);

_λA_ 2 = rate of change in _A_ 2 per unit increase in FFS, from Exhibit 1230 (mi/h); and

_FFSadj_ = adjusted free-flow speed (mi/h).


The curvilinear portion of the speed–flow curve during times when the
adjacent general purpose lane density is less than or equal to 35 pc/mi/ln is
computed from Equation 12-17:


**Equation 12-17:**


where

_S_ 2 = speed drop within the curvilinear portion of the speed–flow
curve (mi/h);

_S_ 1, _BP_ = speed at the breakpoint of the speed–flow curve, calculated
from Equation 12-15 by setting _vp_ to _BP_ (mi/h);

_cadj_ = adjusted basic managed lane segment capacity (pc/h/ln);

_Kcnf_ = density at capacity, without the frictional effect of the adjacent
general purpose lane, from Exhibit 12-30 (pc/mi/ln);

_BP_ = breakpoint in the speed–flow curve separating the linear and
curvilinear sections (pc/h/ln);


⎧

⎨



_A_ 2 = speed reduction per unit of flow rate in the curvilinear section
of the speed–flow curve (mi/h); and



_vp_ = 15-min average flow rate (pc/h/ln).



Continuous access and Buffer 1 segment types operate at lower speeds
when adjacent general purpose lane density is greater than 35 pc/mi/ln. The
indicator variable _Ic_ is used to determine the status of the general purpose
lane operation. This variable is determined by using Equation 12-18.



**Equation 12-18:**



⎧

⎨



where _KGP_ is the density of the adjacent general purpose lane (pc/mi/ln).



The additional speed reduction that occurs in the curvilinear portion of
the speed–flow curve because of high density in the adjacent general purpose
lanes is computed by Equation 12-19:



**Equation 12-19:**



where _Kcf_ is the density at capacity, with the frictional effect of the adjacent
general purpose lane (pc/mi/ln), from Exhibit 12-30, and other variables are
as defined previously.



Exhibit 12-30 tabulates the parameters used by speed computations for
the different basic managed lane segment types.



**Exhibit 12-30: Parameters for Basic Managed Lane Segment Analysis**


|Segment Type|BP λ c λ λ A<br>75 BP 75 c A2 1|
|---|---|
|Continuous access<br>Buffer 1<br>Buffer 2<br>Barrier 1<br>Barrier 2|500<br>0<br>1,800<br>10<br>2.5<br>0<br>0<br>30<br>45<br>600<br>0<br>1,700<br>10<br>1.4<br>0<br>0.0033<br>30<br>42_a_<br>500<br>10<br>1,850<br>10<br>1.5<br>0.02<br>0<br>45_a_<br>NA<br>800<br>0<br>1,750<br>10<br>1.4<br>0<br>0.004<br>35<br>NA<br>700<br>20<br>2,100<br>10<br>1.3<br>0.02<br>0<br>45<br>NA|


Note: _a_ These are average values of density at capacity observed by NCHRP Project 03-96
( _1_ ), ranging from 40.9 to 42.5 pc/mi/ln for Buffer 1 and from 40.1 to 50.4 pc/mi/ln for
Buffer 2 segment types.


**BICYCLE METHODOLOGY FOR MULTILANE HIGHWAYS**


**Bicycle LOS Criteria**

Bicycle levels of service for multilane highway segments are based on a
bicycle LOS score, which is in turn based on a traveler perception model.
Chapter 15, Two-Lane Highways, provides details about this service
measure, which is identical for two-lane highways and multilane highways.
The bicycle LOS score is based, in order of importance, on five variables:

   - Average effective width of the outside through lane,

   - Motorized vehicle volumes and speeds,

   - Heavy vehicle (truck) volumes, and

   - Pavement condition.

The LOS ranges for bicycles on two-lane and multilane highways are
given in Exhibit 12-31.


**Exhibit 12-31: Bicycle LOS for Two-Lane and Multilane Highways**


**LOS** **Bicycle LOS Score**



A
B
C
D
E
F



≤1.5
>1.5–2.5
>2.5–3.5
>3.5–4.5
>4.5–5.5
>5.5


**Required Input Data**

The data required for evaluating bicycle LOS on a multilane highway and
the ranges of values used in the development of the LOS model ( _22_ ) are as
follows:

   - Width of the outside through lane: 10 to 16 ft,

   - Shoulder width: 0 to 6 ft,

   - Motorized vehicle volumes: up to 36,000 annual average daily traffic
(AADT),

   - Number of directional through lanes,

   - Posted speed: 25 to 50 mi/h,

   - Heavy vehicle percentage: 0% to 2%, and

   - Pavement condition: 2 to 5 on the FHWA 5-point pavement rating
scale ( _23_ ).


**Methodology**

The calculation of bicycle LOS on multilane and two-lane highways
shares the same methodology, since multilane and two-lane highways operate
in fundamentally the same manner for bicyclists and motorized vehicle
drivers. Bicyclists travel much more slowly than the prevailing traffic flow
and stay as far to the right as possible, and they use paved shoulders when
available. This similarity indicates the need for only one model.

The bicycle LOS model for multilane highways uses a traveler
perception index calibrated by using a linear regression model. The model
fits independent variables associated with roadway characteristics to the
results of a user survey that rates the comfort of various bicycle facilities.
The resulting bicycle LOS index computes a numerical LOS score, generally
ranging from 0.5 to 6.5, which is stratified to produce a LOS A to F result by
using Exhibit 12-31.

Full details on the bicycle LOS methodology and calculation procedures
are given in Chapter 15.


**Limitations**

The bicycle methodology was developed with data collected on urban
and suburban streets, including facilities that would be defined as suburban
multilane highways. Although the methodology has been successfully applied
to rural multilane highways in different parts of the United States, users
should be aware that conditions on many rural multilane highways (i.e.,
posted speeds of 55 mi/h or higher or heavy vehicle percentages over 2%)
will be outside the range of values used to develop the bicycle LOS model.


## **5. APPLICATIONS**

**EXAMPLE PROBLEMS**

Section 7 of Chapter 26, Freeway and Highway Segments: Supplemental,
provides seven example problems that go through each of the computational
steps involved in applying the automobile to basic freeway and multilane
highway segments:

1. Four-lane freeway LOS (operational analysis),

2. Number of lanes required to achieve a target LOS (design analysis),

3. Six-lane freeway LOS and capacity (operational and planning
analysis),

4. LOS on a five-lane multilane highway with a TWLTL (operational
analysis),

5. Estimation of the mixed-flow operational performance of a basic
segment with a high truck percentage (operational analysis),

6. Severe weather effects on a basic freeway segment (operational
analysis), and

7. Basic managed lane segment with and without friction effects
(operational analysis).

Section 7 of Chapter 26 also provides an example of the application of
the bicycle LOS method, which can be used with multilane highway
segments.


**RELATED CONTENT IN THE HCMAG**

The _Highway Capacity Manual Applications Guide_ (HCMAG) _,_
accessible through the online HCM Volume 4, provides guidance on applying
the HCM on basic freeway segments. Case Study 4 goes through the process
of identifying the goals, objectives, and analysis tools for investigating LOS


on a 3-mi section of New York State Route 7 in Albany. The case study
applies the analysis tools to assess the performance of the route, to identify
areas that are deficient, and to investigate alternatives for correcting the
deficiencies.

This case study includes the following problems related to basic freeway
segments:

1. Problem 1: Analysis of two basic freeway segments

a. Subproblem 1a: Traffic flow patterns

b. Subproblem 1b: Selection of appropriate data and basic freeway
analysis

c. Subproblem 1c: Basic freeway analysis

2. Problem 4: Analysis of segments as part of an extended freeway
facility

a. Subproblem 4a: Separation of Route 7 for HCM analysis

b. Subproblem 4b: Study of off-peak periods

c. Subproblem 4c: What is the operational performance of Route 7
during the peak period?

Although the HCMAG was based on the HCM2000’s procedures and
chapter organization, the general thought process described in its case studies
is applicable to this edition of the HCM.


**EXAMPLE RESULTS**

This section presents the results of applying this chapter’s method in
typical situations. Analysts can use the illustrative results presented in this
section to observe the sensitivity of output performance measures to various
inputs, as well as to help evaluate whether their analysis results are
reasonable. The exhibits in this section are not intended to substitute for an
analysis and are deliberately provided in a format large enough to depict
general trends in the results but not large enough to pull out specific results.


**Sensitivity of Freeway Results to Total Ramp Density and Right-**
**Side Lateral Clearance**

Exhibit 12-32 illustrates how FFS varies for a basic freeway segment
with a base FFS of 75 mi/h when the total ramp density varies from 1 to 4
ramps/mi. The top curve shows the case with adequate right-side clearance
(i.e., 6 ft or greater), while the lower curve shows the case with no right-side
clearance (i.e., no right shoulder).


**Exhibit 12-32: Illustrative Effect of Total Ramp Density and Right-Side Lateral**
**Clearance on Basic Freeway Segment FFS**


Note: Calculated by using this chapter’s methods. Fixed values include BFFS = 75.4 mi/h
for a basic freeway segment and _fLW_ = 6.6 for 10-ft lanes.


A freeway with 2 ramps/mi represents a case where there are 6 ramps
within 3 mi on either side of the study location. This occurs primarily in
urban areas, where interchanges may be close to each other, sometimes even
in excess of 6 ramps/mi. The FFS for that condition is nearly 70 mi/h,
assuming a base FFS of 75 mi/h. In contrast, the same segment without any
right-side clearance has a much lower FFS—just above 60 mi/h.


In general, most interchanges involve two to four ramps. A full
cloverleaf, for example, has four ramps: two on-ramps and two off-ramps in
each direction. A diamond interchange has two ramps in each direction: one
on-ramp and one off-ramp. Thus, a freeway with two cloverleaf interchanges
fully contained within 1 mi would have a total ramp density of 8 ramps/mi. A
freeway with two diamond interchanges fully contained within 1 mi would
have a total density of 4 ramps/mi. This suggests that in any given situation
(with comparable demand flows), cloverleaf interchanges will have a
greater negative impact on FFS than diamond interchanges.

Although the curves in Exhibit 12-32 are not straight lines, their slopes
are relatively constant. On average, an increase of 2 ramps/mi in total ramp
density causes a drop in FFS of approximately 5 mi/h. A reduction in FFS, of
course, implies reductions in capacity and service volumes.


**Sensitivity of Freeway Results to** _**v/c**_ **Ratio**

Exhibit 12-33 shows the relationship between speed and _v/c_ ratio. Not
unexpectedly, the shapes of these curves are similar to the basic speed–flow
curves of Exhibit 12-7. Speed does not begin to decline until a _v/c_ ratio of
0.42 to 0.80 is reached, depending on the FFS.


**Exhibit 12-33: Illustrative Effect of** _**v/c**_ **Ratio on Basic Freeway Segment Speed**


Note: Calculated by using this chapter’s methods. Fixed values include CAF = 1.0, SAF =
1.0, and no heavy vehicle or grade effects.


**Sensitivity of Multilane Highway Results to Access Point**
**Density, Lateral Clearance, and Median Type**

Exhibit 12-34 illustrates the effect of access point density, lateral
clearance, and median type (divided or undivided) on the resulting multilane
highway segment FFS, assuming a base FFS of 65 mi/h.


**Exhibit 12-34: Illustrative Effect of Access Point Density, Lateral Clearance, and**
**Median Type on Multilane Highway Segment FFS**


Note: Calculated by using this chapter’s methods. Fixed values include base FFS = 65 mi/h
and _fLW_ = 0 for 12-ft lanes.


Exhibit 12-34 shows that adding a single access point per mile results in
a 1-mi/h drop in the FFS. This value represents the slope of all four lines in
the exhibit. The effect of lateral clearance is also significant; the FFS is
reduced by nearly 4 mi/h when all other parameters are held fixed. Finally,
the FFS of a divided segment is 1.6 mi/h higher than that of an undivided
segment when clearances and the number of access points are both controlled
for.


**Sensitivity of Freeway Results to Incidents and Inclement**
**Weather**

The speed–flow curves presented in this chapter are primarily sensitive
to flow rates, FFS, and capacity. Incidents and inclement weather reduce a
basic freeway segment’s capacity and therefore indirectly reduce its FFS.
Inclement weather also produces a direct reduction in FFS. Exhibit 12-35
shows speed–flow curves for a basic freeway segment for three different


conditions—base condition, shoulder-closure incident, and heavy snow—for
a base FFS of 70 mi/h. The CAFs used for shoulder closure and heavy snow
are 0.85 and 0.776, respectively, on the basis of default values from Chapter
11, while the SAF for heavy snow is 0.88.


**Exhibit 12-35: Illustrative Effect of Incidents and Inclement Weather on Basic Freeway**
**Segment FFS**


Note: Calculated by using this chapter’s methods. Fixed values include FFS = 70 mi/h, CAF
= 1.0 for base case, SAF = 1.0 for base case, and no heavy vehicle or grade effects.


**Sensitivity of Managed Lane Results to Inclement Weather and**
**General Purpose Lane Friction**

Exhibit 12-36 depicts speed–flow curves for a single-lane continuous
access managed lane segment for combinations of weather (light snow and
nonsevere) and adjacent general purpose lane density (=35 pc/mi/ln,
resulting in no friction, and >35 pc/mi/ln, resulting in friction). The CAF for
light snow is 0.957 and the SAF for light snow is 0.94, on the basis of default
values from Chapter 11.


**Exhibit 12-36: Illustrative Effect of Inclement Weather and General Purpose Lane**
**Friction on Managed Lane FFS**


Note: Calculated by using this chapter’s methods. Fixed values include FFS = 60 mi/h, CAF
= 1.0 for base case, SAF = 1.0 for base case, and no heavy vehicle or grade effects.


**PLANNING AND PRELIMINARY ENGINEERING ANALYSIS**

A frequent objective of planning or preliminary engineering analysis is to
develop a general idea of the number of lanes that will be required to deliver
a target LOS. The primary differences are that many default values will be
used and the demand volume will usually be expressed as an AADT. Thus, a
planning and preliminary engineering analysis starts by converting the
demand expressed as an AADT to an estimate of the directional peak-hour
demand volume (DDHV) with Equation 12-20:


**Equation 12-20:**


where _K_ is the proportion of AADT occurring during the peak hour and _D_ is
the proportion of peak-hour volume traveling in the peak direction.

On urban freeways, the typical range of _K_ -factors is from 0.08 to 0.10.
On rural freeways, values typically range between 0.09 and 0.13. Directional
distributions also vary, as illustrated in Chapter 3, Modal Characteristics, but
a typical value for both urban and rural freeways is 0.55. As with all default
values, locally or regionally calibrated values are preferred and yield more
accurate results. Both the _K-_ factor and the _D_ -factor have a significant impact
on the estimated hourly demand volume.

Once the hourly demand volume is estimated, the methodology follows
the same path as that for design analysis, described next. Additional details
and discussion on planning applications can be found in the _Planning and_
_Preliminary Engineering Applications Guide to the HCM_ in Volume 4 _._


**DESIGN ANALYSIS**

In design analysis, a known demand volume is used to determine the
number of lanes needed to deliver a target LOS. Two modifications are
required to the operational analysis methodology. First, since the number of
lanes is to be determined, the demand volume is converted to a demand flow
rate in passenger cars per hour, not per lane, by using Equation 12-21 instead
of Equation 12-9:


**Equation 12-21:**


where _v_ is the demand flow rate in passenger cars per hour and all other
variables are as previously defined.

Second, a maximum service flow rate for the target LOS is then selected
from Exhibit 12-37 for basic freeway segments or Exhibit 12-38 for
multilane highways. These values are selected from the base speed–flow
curves of Exhibit 12-6 for each LOS. In using these exhibits, the FFS should
be rounded to the nearest 5 mi/h, and no interpolation is permitted.


**Exhibit 12-37: Maximum Service Flow Rates for Basic Freeway Segments Under Base**
**Conditions**







|FFS (mi/h)|Maximum Service Flow Rates for Target LOS (pc/h/ln)<br>A B C D E|
|---|---|
|75<br>70<br>65<br>60<br>55|820<br>1,330<br>1,780<br>2,130<br>2,400<br>770<br>1,260<br>1,730<br>2,110<br>2,400<br>710<br>1,170<br>1,660<br>2,060<br>2,350<br>660<br>1,080<br>1,560<br>2,000<br>2,300<br>600<br>990<br>1,430<br>1,910<br>2,250|


Note: All values rounded to the nearest 10 pc/h/ln.


**Exhibit 12-38: Maximum Service Flow Rates for Multilane Highway Segments Under**
**Base Conditions**







|FFS (mi/h)|Maximum Service Flow Rates for Target LOS (pc/h/ln)<br>A B C D E|
|---|---|
|60<br>55<br>50<br>45|660<br>1,080<br>1,530<br>1,890<br>2,200<br>600<br>990<br>1,430<br>1,790<br>2,100<br>550<br>900<br>1,300<br>1,680<br>2,000<br>490<br>810<br>1,170<br>1,550<br>1,900|


Next, the number of lanes required to deliver the target LOS can be found
from Equation 12-22:


**Equation 12-22:**


where _N_ is the number of lanes required (ln) and _MSFi_ is the maximum
service flow rate for LOS _i_ (pc/h/ln) from Exhibit 12-37 or Exhibit 12-38.

Equation 12-21 and Equation 12-22 can be conveniently combined as
Equation 12-23:


**Equation 12-23:**


where all variables are as previously defined.

The value of _N_ resulting from Equation 12-22 or Equation 12-23 will
most likely be fractional. Since only integer numbers of lanes can be
constructed, the result is always rounded to the next-higher value. Thus, if the
result is 3.2 lanes, 4 must be provided. The 3.2 lanes is, in effect, the
minimum number of lanes needed to provide the target LOS. If the result
were rounded to 3, a poorer LOS than the target value would result.

The rounding-up process will occasionally produce an interesting result:
a target LOS (for example, LOS C) may not be achievable for a given
demand volume. If 2.1 lanes are required to produce LOS C, providing 2
lanes would drop the LOS, most likely to D. However, if three lanes are
provided, the LOS might improve to B. Some judgment may be required to
interpret the results. In this case, two lanes might be provided even though
they would result in a borderline LOS D. Economic considerations might
lead a decision maker to accept a lower operating condition than that
originally targeted.


**SERVICE FLOW RATES, SERVICE VOLUMES, AND DAILY**
**SERVICE VOLUMES**

This chapter’s methodology can be easily manipulated to produce service
flow rates, service volumes, and daily service volumes for basic freeway
segments and multilane highways.

Exhibit 12-37 gave values of the maximum hourly service flow rates
_MSFi_ for each LOS for freeways of varying FFS. These values are given in
terms of passenger cars per hour per lane under equivalent base conditions.
A service flow rate _SFi_ is the maximum rate of flow that can exist while LOS
_i_ is maintained during the 15-min analysis period under prevailing
conditions. It can be computed from the maximum service flow rate by using
Equation 12-24:


**Equation 12-24:**


where all variables are as previously defined.

A service flow rate can be converted to a service volume _SVi_ by
applying a PHF, as shown in Equation 12-25. A service volume is the
maximum hourly volume that can exist while LOS _i_ is maintained during the
worst 15-min period of the analysis hour.


**Equation 12-25:**


where all variables are as previously defined.

A daily service volume _DSVi_ is the maximum AADT that can be
accommodated by the facility under prevailing conditions while LOS _i_ is
maintained during the worst 15-min period of the analysis day. It is estimated
from Equation 12-26:


**Equation 12-26:**


where all variables are as previously defined.

Service flow rates _SF_ and service volumes _SV_ are stated for a single
direction. Daily service volumes _DSV_ are stated as total volumes in _both_
directions of the freeway or multilane highway.

This method can also be used to develop daily service volume tables for
basic managed lane segments by using regional assumptions about the
various input parameters.


**Generalized Daily Service Volumes for Basic Freeway Segments**


Exhibit 12-39 and Exhibit 12-40 show generalized daily service volume
tables for urban and rural basic freeway segments, respectively. They are
based on the following set of typical conditions:

   - Percent heavy vehicles = 5% (urban), 12% (rural);

   - FFS = 70 mi/h; and

   - PHF = 0.94.

Values of rural and urban daily service volumes are provided for fourlane, six-lane, and eight-lane freeways in level and rolling terrain. A range of
_K_ - and _D_ -factors is provided. Users should enter Exhibit 12-39 and Exhibit
12-40 with local or regional values of these factors for the appropriate size
of freeway in the appropriate terrain.


**Exhibit 12-39: Daily Service Volume Table for Urban Basic Freeway Segments (1,000**
**veh/day)**












|K D|Four-Lane Freeways LOS B LOS C LOS D LOS E|Six-Lane Freeways LOS B LOS C LOS D LOS E Level Terrain|Eight-Lane Fre LOS B LOS C LO D|
|---|---|---|---|
|0.08<br>0.50<br>0.55<br>0.60<br>0.65|56.4<br>77.4<br>94.4<br>107.4<br>51.3<br>70.4<br>85.9<br>97.7<br>47.0<br>64.5<br>78.7<br>89.5<br>43.4<br>59.6<br>72.7<br>82.6|84.6<br>116.2<br>141.7<br>161.1<br>76.9<br>105.6<br>128.8<br>146.5<br>70.5<br>96.8<br>118.1<br>134.3<br>65.1<br>89.4<br>109.0<br>124.0|112.8<br>154.9<br>188<br>102.5<br>140.8<br>171<br>94.0<br>129.1<br>157<br>86.8<br>119.1<br>145<br>100.3<br>137.7<br>167<br>91.2<br>125.2<br>152<br>83.6<br>114.7<br>139<br>77.1<br>105.9<br>129<br>90.2<br>123.9<br>151<br>82.0<br>112.6<br>137<br>75.2<br>103.3<br>125<br>69.4<br>95.3<br>116<br>82.0<br>112.6<br>137<br>74.6<br>102.4<br>124<br>68.4<br>93.9<br>114<br>63.1<br>86.6<br>105<br>75.2<br>103.3<br>125<br>68.4<br>93.9<br>114<br>62.7<br>86.0<br>104<br>57.8<br>79.4<br>96|
|0.09 0.50<br>0.55<br>0.60<br>0.65<br>0.10<br>0.50<br>0.55<br>0.60<br>0.65|50.1<br>68.8<br>84.0<br>95.5<br>45.6<br>62.6<br>76.3<br>86.8<br>41.8<br>57.4<br>70.0<br>79.6<br>38.6<br>52.9<br>64.6<br>73.5|75.2<br>103.3<br>125.9<br>143.2<br>68.4<br>93.9<br>114.5<br>130.2<br>62.7<br>86.0<br>104.9<br>119.4<br>57.8<br>79.4<br>96.9<br>110.2|75.2<br>103.3<br>125.9<br>143.2<br>68.4<br>93.9<br>114.5<br>130.2<br>62.7<br>86.0<br>104.9<br>119.4<br>57.8<br>79.4<br>96.9<br>110.2|
|0.09 0.50<br>0.55<br>0.60<br>0.65<br>0.10<br>0.50<br>0.55<br>0.60<br>0.65|45.1<br>62.0<br>75.6<br>85.9<br>41.0<br>56.3<br>68.7<br>78.1<br>37.6<br>51.6<br>63.0<br>71.6<br>34.7<br>47.7<br>58.1<br>66.1|67.7<br>92.9<br>113.3<br>128.9<br>61.5<br>84.5<br>103.0<br>117.2<br>56.4<br>77.4<br>94.4<br>107.4<br>52.1<br>71.5<br>87.2<br>99.2|67.7<br>92.9<br>113.3<br>128.9<br>61.5<br>84.5<br>103.0<br>117.2<br>56.4<br>77.4<br>94.4<br>107.4<br>52.1<br>71.5<br>87.2<br>99.2|
|0.11<br>0.50<br>0.55<br>0.60<br>0.65|41.0<br>56.3<br>68.7<br>78.1<br>37.3<br>51.2<br>62.4<br>71.0<br>34.2<br>46.9<br>57.2<br>65.1<br>31.6<br>43.3<br>52.8<br>60.1|61.5<br>84.5<br>103.0<br>117.2<br>55.9<br>76.8<br>93.7<br>106.5<br>51.3<br>70.4<br>85.9<br>97.7<br>47.3<br>65.0<br>79.3<br>90.1|61.5<br>84.5<br>103.0<br>117.2<br>55.9<br>76.8<br>93.7<br>106.5<br>51.3<br>70.4<br>85.9<br>97.7<br>47.3<br>65.0<br>79.3<br>90.1|
|0.12<br>0.50<br>0.55<br>0.60<br>0.65|37.6<br>51.6<br>63.0<br>71.6<br>34.2<br>46.9<br>57.2<br>65.1<br>31.3<br>43.0<br>52.5<br>59.7<br>28.9<br>39.7<br>48.4<br>55.1|56.4<br>77.4<br>94.4<br>107.4<br>51.3<br>70.4<br>85.9<br>97.7<br>47.0<br>64.5<br>78.7<br>89.5<br>43.4<br>59.6<br>72.7<br>82.6|56.4<br>77.4<br>94.4<br>107.4<br>51.3<br>70.4<br>85.9<br>97.7<br>47.0<br>64.5<br>78.7<br>89.5<br>43.4<br>59.6<br>72.7<br>82.6|


|Col1|Col2|Rolling Terrain|Col4|
|---|---|---|---|
|0.08<br>0.50<br>0.55<br>0.60<br>0.65|53.8<br>73.9<br>90.2<br>102.5<br>48.9<br>67.2<br>82.0<br>93.2<br>44.9<br>61.6<br>75.1<br>85.5<br>41.4<br>56.9<br>69.3<br>78.9|80.8<br>110.9<br>135.2<br>153.8<br>73.4<br>100.8<br>122.9<br>139.8<br>67.3<br>92.4<br>112.7<br>128.2<br>62.1<br>85.3<br>104.0<br>118.3|107.7<br>147.8<br>180<br>97.9<br>134.4<br>163<br>89.7<br>123.2<br>150<br>82.8<br>113.7<br>138<br>95.7<br>131.4<br>160<br>87.0<br>119.5<br>145<br>79.8<br>109.5<br>133<br>73.6<br>101.1<br>123<br>86.1<br>118.3<br>144<br>78.3<br>107.5<br>131<br>71.8<br>98.6<br>120<br>66.3<br>91.0<br>111<br>78.3<br>107.5<br>131<br>71.2<br>97.7<br>119<br>65.3<br>89.6<br>109<br>60.2<br>82.7<br>100<br>71.8<br>98.6<br>120<br>65.3<br>89.6<br>109<br>59.8<br>82.1<br>100<br>55.2<br>75.8<br>92|
|0.09<br>0.50<br>0.55<br>0.60<br>0.65|47.9<br>65.7<br>80.1<br>91.2<br>43.5<br>59.7<br>72.9<br>82.9<br>39.9<br>54.8<br>66.8<br>76.0<br>36.8<br>50.5<br>61.6<br>70.1|71.8<br>98.6<br>120.2<br>136.7<br>65.3<br>89.6<br>109.3<br>124.3<br>59.8<br>82.1<br>100.2<br>113.9<br>55.2<br>75.8<br>92.5<br>105.2|71.8<br>98.6<br>120.2<br>136.7<br>65.3<br>89.6<br>109.3<br>124.3<br>59.8<br>82.1<br>100.2<br>113.9<br>55.2<br>75.8<br>92.5<br>105.2|
|0.10<br>0.50<br>0.55<br>0.60<br>0.65|43.1<br>59.1<br>72.1<br>82.0<br>39.2<br>53.8<br>65.6<br>74.6<br>35.9<br>49.3<br>60.1<br>68.4<br>33.1<br>45.5<br>55.5<br>63.1|64.6<br>88.7<br>108.2<br>123.1<br>58.7<br>80.6<br>98.4<br>111.9<br>53.8<br>73.9<br>90.2<br>102.5<br>49.7<br>68.2<br>83.2<br>94.7|64.6<br>88.7<br>108.2<br>123.1<br>58.7<br>80.6<br>98.4<br>111.9<br>53.8<br>73.9<br>90.2<br>102.5<br>49.7<br>68.2<br>83.2<br>94.7|
|0.11<br>0.50<br>0.55<br>0.60<br>0.65|39.2<br>53.8<br>65.6<br>74.6<br>35.6<br>48.9<br>59.6<br>67.8<br>32.6<br>44.8<br>54.6<br>62.1<br>30.1<br>41.4<br>50.4<br>57.4|58.7<br>80.6<br>98.4<br>111.9<br>53.4<br>73.3<br>89.4<br>101.7<br>48.9<br>67.2<br>82.0<br>93.2<br>45.2<br>62.0<br>75.7<br>86.1|58.7<br>80.6<br>98.4<br>111.9<br>53.4<br>73.3<br>89.4<br>101.7<br>48.9<br>67.2<br>82.0<br>93.2<br>45.2<br>62.0<br>75.7<br>86.1|
|0.12<br>0.50<br>0.55<br>0.60<br>0.65|35.9<br>49.3<br>60.1<br>68.4<br>32.6<br>44.8<br>54.6<br>62.1<br>29.9<br>41.1<br>50.1<br>57.0<br>27.6<br>37.9<br>46.2<br>52.6|53.8<br>73.9<br>90.2<br>102.5<br>48.9<br>67.2<br>82.0<br>93.2<br>44.9<br>61.6<br>75.1<br>85.5<br>41.4<br>56.9<br>69.3<br>78.9|53.8<br>73.9<br>90.2<br>102.5<br>48.9<br>67.2<br>82.0<br>93.2<br>44.9<br>61.6<br>75.1<br>85.5<br>41.4<br>56.9<br>69.3<br>78.9|



Note: Key assumptions: 5% trucks, PHF = 0.94, FFS = 70 mi/h.


**Exhibit 12-40: Daily Service Volume Table for Rural Basic Freeway Segments (1,000**
**veh/day)**










|K D|Four-Lane Freeways LOS B LOS C LOS D LOS E|Six-Lane Freeways LOS B LOS C LOS D LOS E Level Terrain|Eight-Lane Free LOS B LOS C LOS D|
|---|---|---|---|
|0.08<br>0.50<br>0.55<br>0.60<br>0.65|52.9<br>72.6<br>88.5<br>100.7<br>48.1<br>66.0<br>80.5<br>91.6<br>44.1<br>60.5<br>73.8<br>83.9<br>40.7<br>55.8<br>68.1<br>77.5|79.3<br>108.9<br>132.8<br>151.1<br>72.1<br>99.0<br>120.7<br>137.3<br>66.1<br>90.7<br>110.7<br>125.9<br>61.0<br>83.8<br>102.2<br>116.2|105.8<br>145.2<br>177.<br>96.1<br>132.0<br>161.0<br>88.1<br>121.0<br>147.6<br>81.3<br>111.7<br>136.2<br>94.0<br>129.1<br>157.4<br>85.5<br>117.3<br>143.1<br>78.3<br>107.6<br>131.2<br>72.3<br>99.3<br>121.1<br>84.6<br>116.2<br>141.7<br>76.9<br>105.6<br>128.8<br>70.5<br>96.8<br>118.1<br>65 1<br>89 4<br>109 0|
|0.09<br>0.50<br>0.55<br>0.60<br>0.65|47.0<br>64.5<br>78.7<br>89.5<br>42.7<br>58.7<br>71.6<br>81.4<br>39.2<br>53.8<br>65.6<br>74.6<br>36.2<br>49.6<br>60.5<br>68.9|70.5<br>96.8<br>118.1<br>134.3<br>64.1<br>88.0<br>107.3<br>122.1<br>58.8<br>80.7<br>98.4<br>111.9<br>54.2<br>74.5<br>90.8<br>103.3|70.5<br>96.8<br>118.1<br>134.3<br>64.1<br>88.0<br>107.3<br>122.1<br>58.8<br>80.7<br>98.4<br>111.9<br>54.2<br>74.5<br>90.8<br>103.3|
|0.10<br>0.50<br>0.55<br>0.60<br>0 65|42.3<br>58.1<br>70.8<br>80.6<br>38.5<br>52.8<br>64.4<br>73.2<br>35.3<br>48.4<br>59.0<br>67.1<br>32 5<br>44 7<br>54 5<br>62 0|63.5<br>87.1<br>106.3<br>120.9<br>57.7<br>79.2<br>96.6<br>109.9<br>52.9<br>72.6<br>88.5<br>100.7<br>48 8<br>67 0<br>81 7<br>93 0|63.5<br>87.1<br>106.3<br>120.9<br>57.7<br>79.2<br>96.6<br>109.9<br>52.9<br>72.6<br>88.5<br>100.7<br>48 8<br>67 0<br>81 7<br>93 0|


|0.65|32.5 44.7 54.5 62.0|48.8 67.0 81.7 93.0|65.1 89.4 109.0<br>76.9 105.6 128.8<br>69.9 96.0 117.1<br>64.1 88.0 107.3<br>59.2 81.2 99.1<br>70.5 96.8 118.1<br>64.1 88.0 107.3<br>58.8 80.7 98.4<br>54.2 74.5 90.8|
|---|---|---|---|
|0.11<br>0.50<br>0.55<br>0.60<br>0.65|38.5<br>52.8<br>64.4<br>73.2<br>35.0<br>48.0<br>58.5<br>66.6<br>32.0<br>44.0<br>53.7<br>61.0<br>29.6<br>40.6<br>49.5<br>56.3|57.7<br>79.2<br>96.6<br>109.9<br>52.4<br>72.0<br>87.8<br>99.9<br>48.1<br>66.0<br>80.5<br>91.6<br>44.4<br>60.9<br>74.3<br>84.5|57.7<br>79.2<br>96.6<br>109.9<br>52.4<br>72.0<br>87.8<br>99.9<br>48.1<br>66.0<br>80.5<br>91.6<br>44.4<br>60.9<br>74.3<br>84.5|
|0.12<br>0.50<br>0.55<br>0.60<br>0.65|35.3<br>48.4<br>59.0<br>67.1<br>32.0<br>44.0<br>53.7<br>61.0<br>29.4<br>40.3<br>49.2<br>56.0<br>27.1<br>37.2<br>45.4<br>51.6|52.9<br>72.6<br>88.5<br>100.7<br>48.1<br>66.0<br>80.5<br>91.6<br>44.1<br>60.5<br>73.8<br>83.9<br>40.7<br>55.8<br>68.1<br>77.5|52.9<br>72.6<br>88.5<br>100.7<br>48.1<br>66.0<br>80.5<br>91.6<br>44.1<br>60.5<br>73.8<br>83.9<br>40.7<br>55.8<br>68.1<br>77.5|








|0.12 0.55 0.60 0.65|32.0 44.0 53.7 61.0 29.4 40.3 49.2 56.0 27.1 37.2 45.4 51.6|48.1 66.0 80.5 91.6 44.1 60.5 73.8 83.9 40.7 55.8 68.1 77.5 Rolling Terrain|64.1 88.0 107.3 58.8 80.7 98.4 54.2 74.5 90.8|
|---|---|---|---|
|0.08<br>0.50<br>0.55<br>0.60<br>0.65|47.8<br>65.6<br>80.0<br>91.0<br>43.4<br>59.6<br>72.7<br>82.7<br>39.8<br>54.6<br>66.6<br>75.8<br>36.7<br>50.4<br>61.5<br>70.0|71.6<br>98.4<br>120.0<br>136.5<br>65.1<br>89.4<br>109.1<br>124.0<br>59.7<br>82.0<br>100.0<br>113.7<br>55.1<br>75.7<br>92.3<br>105.0|95.5<br>131.1<br>160.0<br>86.8<br>119.2<br>145.4<br>79.6<br>109.3<br>133.3<br>73.5<br>100.9<br>123.0<br>84.9<br>116.6<br>142.2<br>77.2<br>106.0<br>129.3<br>70.8<br>97.1<br>118.5<br>65.3<br>89.7<br>109.4<br>76.4<br>104.9<br>128.0<br>69.5<br>95.4<br>116.3<br>63.7<br>87.4<br>106.6<br>58.8<br>80.7<br>98.4<br>69.5<br>95.4<br>116.3<br>63.2<br>86.7<br>105.8<br>57.9<br>79.5<br>96.9<br>53.4<br>73.4<br>89.5<br>63.7<br>87.4<br>106.6<br>57.9<br>79.5<br>96.9<br>53.1<br>72.9<br>88.9<br>49.0<br>67.3<br>82.0|
|0.09<br>0.50<br>0.55<br>0.60<br>0.65|42.5<br>58.3<br>71.1<br>80.9<br>38.6<br>53.0<br>64.6<br>73.5<br>35.4<br>48.6<br>59.2<br>67.4<br>32.7<br>44.8<br>54.7<br>62.2|63.7<br>87.4<br>106.6<br>121.3<br>57.9<br>79.5<br>96.9<br>110.3<br>53.1<br>72.9<br>88.9<br>101.1<br>49.0<br>67.3<br>82.0<br>93.3|63.7<br>87.4<br>106.6<br>121.3<br>57.9<br>79.5<br>96.9<br>110.3<br>53.1<br>72.9<br>88.9<br>101.1<br>49.0<br>67.3<br>82.0<br>93.3|
|0.10 0.50<br>0.55<br>0.60<br>0.65|38.2<br>52.5<br>64.0<br>72.8<br>34.7<br>47.7<br>58.2<br>66.2<br>31.8<br>43.7<br>53.3<br>60.6<br>29.4<br>40.4<br>49.2<br>56.0|57.3<br>78.7<br>96.0<br>109.2<br>52.1<br>71.5<br>87.2<br>99.2<br>47.8<br>65.6<br>80.0<br>91.0<br>44.1<br>60.5<br>73.8<br>84.0|57.3<br>78.7<br>96.0<br>109.2<br>52.1<br>71.5<br>87.2<br>99.2<br>47.8<br>65.6<br>80.0<br>91.0<br>44.1<br>60.5<br>73.8<br>84.0|
|0.11<br>0.50<br>0.55<br>0.60<br>0.65|34.7<br>47.7<br>58.2<br>66.2<br>31.6<br>43.4<br>52.9<br>60.1<br>28.9<br>39.7<br>48.5<br>55.1<br>26.7<br>36.7<br>44.7<br>50.9|52.1<br>71.5<br>87.2<br>99.2<br>47.4<br>65.0<br>79.3<br>90.2<br>43.4<br>59.6<br>72.7<br>82.7<br>40.1<br>55.0<br>67.1<br>76.3|52.1<br>71.5<br>87.2<br>99.2<br>47.4<br>65.0<br>79.3<br>90.2<br>43.4<br>59.6<br>72.7<br>82.7<br>40.1<br>55.0<br>67.1<br>76.3|
|0.12<br>0.50<br>0.55<br>0.60<br>0.65|31.8<br>43.7<br>53.3<br>60.6<br>28.9<br>39.7<br>48.5<br>55.1<br>26.5<br>36.4<br>44.4<br>50.5<br>24.5<br>33.6<br>41.0<br>46.7|47.8<br>65.6<br>80.0<br>91.0<br>43.4<br>59.6<br>72.7<br>82.7<br>39.8<br>54.6<br>66.6<br>75.8<br>36.7<br>50.4<br>61.5<br>70.0|47.8<br>65.6<br>80.0<br>91.0<br>43.4<br>59.6<br>72.7<br>82.7<br>39.8<br>54.6<br>66.6<br>75.8<br>36.7<br>50.4<br>61.5<br>70.0|



Note: Key assumptions: 12% trucks, PHF = 0.94, FFS = 70 mi/h.


**Generalized Daily Service Volumes for Multilane Highways**

Exhibit 12-41 and Exhibit 12-42 are generalized daily service volume
tables for urban and rural multilane highways, respectively. They are based
on the following set of typical conditions:

   - Percent heavy vehicles = 5% (urban), 12% (rural);

   - FFS = 60 mi/h; and


   - PHF = 0.95 (urban), 0.88 (rural).

Daily service volumes are provided for four-, six-, and eight-lane
highways in level and rolling terrain. A range of _K_ - and _D_ -factors is
provided. Users should enter Exhibit 12-41 and Exhibit 12-42 with local or
regional values of these factors for the appropriate size of multilane highway
in the appropriate terrain.


**Exhibit 12-41: Generalized Daily Service Volumes for Urban Multilane Highways (1,000**
**veh/day)**










|K D|Four-Lane Highways LOS B LOS C LOS D LOS E|Six-Lane Highways LOS B LOS C LOS D LOS E Level Terrain|Eight-Lane Highways LOS B LOS C LOS D LOS E|
|---|---|---|---|
|0.08<br>0.50<br>0.55<br>0.60<br>0.65|48.9<br>69.2<br>85.5<br>99.5<br>44.4<br>62.9<br>77.7<br>90.5<br>40.7<br>57.7<br>71.3<br>82.9<br>37.6<br>53.2<br>65.8<br>76.6|73.3<br>103.8 128.3 149.3<br>66.6<br>94.4<br>116.6 135.7<br>61.1<br>86.5<br>106.9 124.4<br>56.4<br>79.9<br>98.7<br>114.8|97.7<br>138.4 171.0 199.0<br>88.8<br>125.8 155.5 181.0<br>81.4<br>115.4 142.5 165.9<br>75.2<br>106.5 131.5 153.1|
|0.09<br>0.50<br>0.55<br>0.60<br>0.65|43.4<br>61.5<br>76.0<br>88.5<br>39.5<br>55.9<br>69.1<br>80.4<br>36.2<br>51.3<br>63.3<br>73.7<br>33.4<br>47.3<br>58.5<br>68.1|65.1<br>92.3<br>114.0 132.7<br>59.2<br>83.9<br>103.6 120.6<br>54.3<br>76.9<br>95.0<br>110.6<br>50.1<br>71.0<br>87.7<br>102.1|86.9<br>123.0 152.0 176.9<br>79.0<br>111.9 138.2 160.8<br>72.4<br>102.5 126.7 147.4<br>66.8<br>94.7<br>116.9 136.1|
|0.10<br>0.50<br>0.55<br>0.60<br>0.65|39.1<br>55.4<br>68.4<br>79.6<br>35.5<br>50.3<br>62.2<br>72.4<br>32.6<br>46.1<br>57.0<br>66.3<br>30.1<br>42.6<br>52.6<br>61.2|58.6<br>83.1<br>102.6 119.4<br>53.3<br>75.5<br>93.3<br>108.6<br>48.9<br>69.2<br>85.5<br>99.5<br>45.1<br>63.9<br>78.9<br>91.9|78.2<br>110.7 136.8 159.2<br>71.1<br>100.7 124.4 144.8<br>65.1<br>92.3<br>114.0 132.7<br>60.1<br>85.2<br>105.2 122.5|
|0.11<br>0.50<br>0.55<br>0.60<br>0.65|35.5<br>50.3<br>62.2<br>72.4<br>32.3<br>45.8<br>56.5<br>65.8<br>29.6<br>41.9<br>51.8<br>60.3<br>27.3<br>38.7<br>47.8<br>55.7|53.3<br>75.5<br>93.3<br>108.6<br>48.5<br>68.6<br>84.8<br>98.7<br>44.4<br>62.9<br>77.7<br>90.5<br>41.0<br>58.1<br>71.7<br>83.5|71.1<br>100.7 124.4 144.8<br>64.6<br>91.5<br>113.1 131.6<br>59.2<br>83.9<br>103.6 120.6<br>54.7<br>77.4<br>95.7<br>111.4|
|0.12<br>0.50<br>0.55<br>0.60<br>0.65|32.6<br>46.1<br>57.0<br>66.3<br>29.6<br>41.9<br>51.8<br>60.3<br>27.1<br>38.5<br>47.5<br>55.3<br>25.1<br>35.5<br>43.8<br>51.0|48.9<br>69.2<br>85.5<br>99.5<br>44.4<br>62.9<br>77.7<br>90.5<br>40.7<br>57.7<br>71.3<br>82.9<br>37.6<br>53.2<br>65.8<br>76.6|65.1<br>92.3<br>114.0 132.7<br>59.2<br>83.9<br>103.6 120.6<br>54.3<br>76.9<br>95.0<br>110.6<br>50.1<br>71.0<br>87.7<br>102.1|


|Col1|Col2|Rolling Terrain|Col4|
|---|---|---|---|
|0.08<br>0.50<br>0.55<br>0.60<br>0.65|46.6<br>66.1<br>81.6<br>95.0<br>42.4<br>60.1<br>74.2<br>86.4<br>38.9<br>55.1<br>68.0<br>79.2<br>35.9<br>50.8<br>62.8<br>73.1|70.0<br>99.1<br>122.4 142.5<br>63.6<br>90.1<br>111.3 129.5<br>58.3<br>82.6<br>102.0 118.8<br>53.8<br>76.2<br>94.2<br>109.6|93.3<br>132.1 163.2 190.0<br>84.8<br>120.1 148.4 172.7<br>77.7<br>110.1 136.0 158.3<br>71.7<br>101.6 125.6 146.2|
|<br>0.50<br>0.55|41.5<br>58.7<br>72.5<br>84.4<br>37.7<br>53.4<br>66.0<br>76.8|62.2<br>88.1<br>108.8 126.7<br>56.5<br>80.1<br>98.9<br>115.2|82.9<br>117.5 145.1 168.9<br>75.4<br>106.8 131.9 153.5|


|0.09|Col2|Col3|Col4|
|---|---|---|---|
|0.09 0.60<br>0.65|34.5<br>48.9<br>60.5<br>70.4<br>31.9<br>45.2<br>55.8<br>65.0|51.8<br>73.4<br>90.7<br>105.6<br>47.8<br>67.8<br>83.7<br>97.4|69.1<br>97.9<br>120.9 140.7<br>63.8<br>90.3<br>111.6 129.9|
|0.10<br>0.50<br>0.55<br>0.60<br>0.65|37.3<br>52.9<br>65.3<br>76.0<br>33.9<br>48.0<br>59.4<br>69.1<br>31.1<br>44.0<br>54.4<br>63.3<br>28.7<br>40.7<br>50.2<br>58.5|56.0<br>79.3<br>97.9<br>114.0<br>50.9<br>72.1<br>89.0<br>103.6<br>46.6<br>66.1<br>81.6<br>95.0<br>43.0<br>61.0<br>75.3<br>87.7|74.6<br>105.7 130.6 152.0<br>67.8<br>96.1<br>118.7 138.2<br>62.2<br>88.1<br>108.8 126.7<br>57.4<br>81.3<br>100.4 116.9|
|0.11<br>0.50<br>0.55<br>0.60<br>0.65|33.9<br>48.0<br>59.4<br>69.1<br>30.8<br>43.7<br>54.0<br>62.8<br>28.3<br>40.0<br>49.5<br>57.6<br>26.1<br>37.0<br>45.7<br>53.1|50.9<br>72.1<br>89.0<br>103.6<br>46.3<br>65.5<br>80.9<br>94.2<br>42.4<br>60.1<br>74.2<br>86.4<br>39.1<br>55.4<br>68.5<br>79.7|67.8<br>96.1<br>118.7 138.2<br>61.7<br>87.4<br>107.9 125.6<br>56.5<br>80.1<br>98.9<br>115.2<br>52.2<br>73.9<br>91.3<br>106.3|
|0.12<br>0.50<br>0.55<br>0.60<br>0.65|31.1<br>44.0<br>54.4<br>63.3<br>28.3<br>40.0<br>49.5<br>57.6<br>25.9<br>36.7<br>45.3<br>52.8<br>23.9<br>33.9<br>41.9<br>48.7|46.6<br>66.1<br>81.6<br>95.0<br>42.4<br>60.1<br>74.2<br>86.4<br>38.9<br>55.1<br>68.0<br>79.2<br>35.9<br>50.8<br>62.8<br>73.1|62.2<br>88.1<br>108.8 126.7<br>56.5<br>80.1<br>98.9<br>115.2<br>51.8<br>73.4<br>90.7<br>105.6<br>47.8<br>67.8<br>83.7<br>97.4|



Note: Key assumptions: 5% trucks, PHF = 0.95, FFS = 60 mi/h.


**Exhibit 12-42: Generalized Daily Service Volumes for Rural Multilane Highways (1,000**
**veh/day)**












|K D|Four-Lane Highways LOS B LOS C LOS D LOS E|Six-Lane Highways LOS B LOS C LOS D LOS E Level Terrain|Eight-Lane Highway LOS B LOS C LOS D L|
|---|---|---|---|
|0.08 0.50<br>0.55<br>0.60<br>0.65|42.4<br>60.1<br>74.3<br>86.4<br>38.6<br>54.6<br>67.5<br>78.6<br>35.4<br>50.1<br>61.9<br>72.0<br>32.6<br>46.2<br>57.1<br>66.5|63.6<br>90.2<br>111.4<br>129.6<br>57.9<br>82.0<br>101.3<br>117.9<br>53.0<br>75.1<br>92.8<br>108.0<br>49.0<br>69.4<br>85.7<br>99.7|84.9<br>120.2<br>148.5<br>1<br>77.1<br>109.3<br>135.0<br>1<br>70.7<br>100.2<br>123.8<br>1<br>65.3<br>92.5<br>114.2<br>1|
|0.09<br>0.50<br>0.55<br>0.60<br>0.65|37.7<br>53.4<br>66.0<br>76.8<br>34.3<br>48.6<br>60.0<br>69.8<br>31.4<br>44.5<br>55.0<br>64.0<br>29.0<br>41.1<br>50.8<br>59.1|56.6<br>80.1<br>99.0<br>115.2<br>51.4<br>72.9<br>90.0<br>104.8<br>47.1<br>66.8<br>82.5<br>96.0<br>43.5<br>61.6<br>76.2<br>88.6|75.4<br>106.9<br>132.0<br>1<br>68.6<br>97.1<br>120.0<br>1<br>62.9<br>89.0<br>110.0<br>1<br>58.0<br>82.2<br>101.5<br>1|
|0.10<br>0.50<br>0.55<br>0.60<br>0.65|33.9<br>48.1<br>59.4<br>69.1<br>30.9<br>43.7<br>54.0<br>62.9<br>28.3<br>40.1<br>49.5<br>57.6<br>26.1<br>37.0<br>45.7<br>53.2|50.9<br>72.1<br>89.1<br>103.7<br>46.3<br>65.6<br>81.0<br>94.3<br>42.4<br>60.1<br>74.3<br>86.4<br>39.2<br>55.5<br>68.5<br>79.8|67.9<br>96.2<br>118.8<br>1<br>61.7<br>87.4<br>108.0<br>1<br>56.6<br>80.1<br>99.0<br>1<br>52.2<br>74.0<br>91.4<br>1|
|0.11<br>0.50<br>0.55<br>0.60<br>0.65|30.9<br>43.7<br>54.0<br>62.9<br>28.1<br>39.7<br>49.1<br>57.1<br>25.7<br>36.4<br>45.0<br>52.4<br>23.7<br>33.6<br>41.5<br>48.4|46.3<br>65.6<br>81.0<br>94.3<br>42.1<br>59.6<br>73.6<br>85.7<br>38.6<br>54.6<br>67.5<br>78.6<br>35.6<br>50.4<br>62.3<br>72.5|61.7<br>87.4<br>108.0<br>1<br>56.1<br>79.5<br>98.2<br>1<br>51.4<br>72.9<br>90.0<br>1<br>47.5<br>67.3<br>83.1|
|0 12<br>0.50<br>0.55|28.3<br>40.1<br>49.5<br>57.6<br>25.7<br>36.4<br>45.0<br>52.4|42.4<br>60.1<br>74.3<br>86.4<br>38.6<br>54.6<br>67.5<br>78.6|56.6<br>80.1<br>99.0<br>1<br>51.4<br>72.9<br>90.0<br>1|


|0.12 0.60 0.65|23.6 33.4 41.3 48.0 21.8 30.8 38.1 44.3|35.4 50.1 61.9 72.0 32.6 46.2 57.1 66.5 Rolling Terrain|47.1 66.8 82.5 43.5 61.6 76.2|
|---|---|---|---|
|0.08<br>0.50<br>0.55<br>0.60<br>0.65|38.3<br>54.3<br>67.1<br>78.1<br>34.8<br>49.4<br>61.0<br>71.0<br>31.9<br>45.2<br>55.9<br>65.1<br>29.5<br>41.8<br>51.6<br>60.0|57.5<br>81.4<br>100.6<br>117.1<br>52.3<br>74.0<br>91.5<br>106.5<br>47.9<br>67.9<br>83.8<br>97.6<br>44.2<br>62.6<br>77.4<br>90.1|76.6<br>108.6<br>134.1<br>1<br>69.7<br>98.7<br>121.9<br>1<br>63.9<br>90.5<br>111.8<br>1<br>59.0<br>83.5<br>103.2<br>1|
|0.09<br>0.50<br>0.55<br>0.60<br>0.65|34.1<br>48.3<br>59.6<br>69.4<br>31.0<br>43.9<br>54.2<br>63.1<br>28.4<br>40.2<br>49.7<br>57.8<br>26.2<br>37.1<br>45.9<br>53.4|51.1<br>72.4<br>89.4<br>104.1<br>46.5<br>65.8<br>81.3<br>94.6<br>42.6<br>60.3<br>74.5<br>86.7<br>39.3<br>55.7<br>68.8<br>80.1|68.1<br>96.5<br>119.2<br>1<br>61.9<br>87.7<br>108.4<br>1<br>56.8<br>80.4<br>99.4<br>1<br>52.4<br>74.2<br>91.7<br>1|
|0.10<br>0.50<br>0.55<br>0.60<br>0.65|30.7<br>43.4<br>53.7<br>62.5<br>27.9<br>39.5<br>48.8<br>56.8<br>25.5<br>36.2<br>44.7<br>52.0<br>23.6<br>33.4<br>41.3<br>48.0|46.0<br>65.1<br>80.5<br>93.7<br>41.8<br>59.2<br>73.2<br>85.2<br>38.3<br>54.3<br>67.1<br>78.1<br>35.4<br>50.1<br>61.9<br>72.1|61.3<br>86.9<br>107.3<br>1<br>55.7<br>79.0<br>97.5<br>1<br>51.1<br>72.4<br>89.4<br>1<br>47.2<br>66.8<br>82.5|
|0.11<br>0.50<br>0.55<br>0.60<br>0.65|27.9<br>39.5<br>48.8<br>56.8<br>25.3<br>35.9<br>44.3<br>51.6<br>23.2<br>32.9<br>40.6<br>47.3<br>21.4<br>30.4<br>37.5<br>43.7|41.8<br>59.2<br>73.2<br>85.2<br>38.0<br>53.8<br>66.5<br>77.4<br>34.8<br>49.4<br>61.0<br>71.0<br>32.2<br>45.6<br>56.3<br>65.5|55.7<br>79.0<br>97.5<br>1<br>50.7<br>71.8<br>88.7<br>1<br>46.5<br>65.8<br>81.3<br>42.9<br>60.7<br>75.0|
|0.12<br>0.50<br>0.55<br>0.60<br>0.65|25.5<br>36.2<br>44.7<br>52.0<br>23.2<br>32.9<br>40.6<br>47.3<br>21.3<br>30.2<br>37.3<br>43.4<br>19.7<br>27.8<br>34.4<br>40.0|38.3<br>54.3<br>67.1<br>78.1<br>34.8<br>49.4<br>61.0<br>71.0<br>31.9<br>45.2<br>55.9<br>65.1<br>29.5<br>41.8<br>51.6<br>60.0|51.1<br>72.4<br>89.4<br>1<br>46.5<br>65.8<br>81.3<br>42.6<br>60.3<br>74.5<br>39.3<br>55.7<br>68.8|



Note: Key assumptions: 12% trucks, PHF = 0.88, FFS = 60 mi/h.


**Appropriate Use of Service Volume Tables**

The preceding service volume tables must be used with care. Because the
characteristics of any given freeway or multilane highway may or may not be
typical, the values should not be used to evaluate a specific freeway or
multilane highway segment. The exhibits are intended to allow a general
evaluation of many facilities within a given jurisdiction on a first-pass basis
to identify segments or facilities that might fail to meet a jurisdiction’s
operating standards. The segments or facilities so identified should then be
evaluated in more detail with this chapter’s core methodology in combination
with each segment’s site-specific characteristics. These service volume
tables should not be used to make final decisions on which segments or
facilities to improve or on specific designs for such improvements.


**USE OF ALTERNATIVE TOOLS**

General guidance for the use of alternative traffic analysis tools for
capacity and LOS analysis is provided in Chapter 6, HCM and Alternative
Analysis Tools. This section contains specific guidance for the application of
alternative tools to the analysis of basic freeway and multilane highway
segments.

Exhibit 12-43 tabulates the HCM limitations for basic freeway and
multilane highway segments along with the potential for improved treatment
by alternative tools.


**Exhibit 12-43: Limitations of HCM Basic Freeway and Multilane Highway Segments**
**Procedure**


**Potential for Improved Treatment by**
**Limitation** **Alternative Tools**



Special lanes reserved for a single vehicle
type, such as truck, and climbing lanes, or
specific lane control treatments to restrict
lane changing



Modeled explicitly by simulation



Extended bridge and tunnel segments Can be approximated by using assumptions
related to desired speed and number of lanes
along each segment
Segments near a toll plaza Can be approximated by using assumptions
related to discharge at toll plaza
Facilities with FFS less than 55 mi/h or Modeled explicitly by simulation
more than 75 mi/h for basic freeway
segments, or less than 45 mi/h or more
than 70 mi/h for multilane highways

Oversaturated conditions (refer to Modeled explicitly by simulation
Chapters 10 and 26 for further discussion)

Influence of downstream blockages or Modeled explicitly by simulation
queuing on a segment



Posted speed limit and extent of police Can be approximated by using assumptions
enforcement related to desired speed along a segment
Presence of ITS features related to Several features modeled explicitly by
vehicle or driver guidance, and active simulation; others may be approximated by
traffic and demand management using assumptions (for example, by modifying
strategies, including ramp metering origin–destination demands by time interval)



Presence of ITS features related to Several features modeled explicitly by
vehicle or driver guidance, and active simulation; others may be approximated by
traffic and demand management using assumptions (for example, by modifying
strategies, including ramp metering origin–destination demands by time interval)

Evaluation of transition zones where a Modeled explicitly by simulation
multilane highway transitions to a two-lane
highway or is interrupted by a traffic signal
or roundabout intersection



Modeled explicitly by simulation


The negative impacts of poor weather
conditions, traffic accidents or incidents,
railroad crossings, or construction
operations on multilane highways

Differences between types of median
barriers and difference between impacts
of a median barrier and a TWLTL on
multilane highways

Significant presence of on-street parking,
bus stops, and pedestrians on multilane
highways



Limited guidance for modeling adverse
conditions on multilane highways in simulation


Limited guidance available for modeling in
simulation


Can be estimated in some simulation tools



As with most other procedural chapters in the HCM, simulation outputs,
especially graphics-based presentations, can provide details on point
problems that might otherwise go unnoticed with a macroscopic analysis that
yields only segment-level measures. The effect of downstream conditions on
lane utilization and backup beyond the segment boundary is a good example
of an analysis that can benefit from the increased insight offered by a
microscopic model.


**Development of HCM-Compatible Performance Measures Using**
**Alternative Tools**

The LOS for basic freeway and multilane highway segments is based on
traffic density expressed in passenger cars per mile per lane. The HCM
methodology estimates density by dividing the flow rate by the average
passenger car speed. Simulation models typically estimate density by
dividing the average number of vehicles in the segment by the area of the
segment (in lane miles). The result is vehicles per lane mile. This
measurement corresponds to density based on space mean speed. The HCMreported density is also based on space mean speed. Generally, increased
speed variability in driver behavior (which simulators usually include)
results in lower average space mean speed and higher density. In obtaining
density from alternative models, the following are important to take into
account:

   - The vehicles included in the density estimation (for example, whether
only the vehicles that have exited the link are considered);

   - The manner in which auxiliary lanes are considered;


   - The units used for density, since a simulation package would
typically provide density in units of vehicles rather than passenger
cars; converting the simulation outputs to passenger cars with the
HCM PCE values is typically not appropriate, given that the
simulation should already account for the effects of heavy vehicles on
a microscopic basis(with heavy vehicles operating at lower speeds
and at longer headways(thus making any additional adjustments
duplicative;

   - The units used in the reporting of density (e.g., whether it is reported
per lane mile);

   - The homogeneity of the analysis segment, since the HCM does not use
the segment length as an input (unless it is a specific upgrade or
downgrade segment, where the length is used to estimate the PCE
values) and conditions are assumed to be homogeneous for the entire
segment; and

   - The driver variability assumed in the simulation package, since
increased driver variability will generally increase the average
density.

The HCM provides capacity estimates in passenger cars per hour per
lane as a function of FFS. To compare the HCM’s estimates with capacity
estimates from a simulation package, the following should be considered:

   - The manner in which a simulation package provides the number of
vehicles exiting a segment; in some cases it may be necessary to
provide virtual detectors at a specific point on the simulated segment
so that the maximum throughput can be obtained;

   - The units used to specify maximum throughput, since a simulation
package would do this in units of vehicles rather than passenger cars;
converting vehicles to passenger cars by using the HCM PCE values
is typically not appropriate, since differences between automobile
and heavy vehicle performance should already be accounted for
microscopically within a simulation; and

   - The incorporation of other simulation inputs, such as the “minimum
separation of vehicles,” that affect the capacity result.


**Conceptual Differences Between HCM and Simulation Modeling**
**That Preclude Direct Comparison of Results**

The HCM methodology is based on the relationship between speed and
flow for various values of FFS. One fundamental potential difference
between the HCM and other models is this relationship. For example, the
HCM assumes a constant speed for a wide range of flows. However, this is
not necessarily the case in simulation packages, some of which assume a
continuously decreasing speed with increasing flow. Furthermore, in some
simulation packages, that relationship can change when certain parameters
(for example, in a car following model) are modified. Therefore,
compatibility of performance measures between the HCM and an alternative
model for a given set of flows does not necessarily guarantee compatibility
for all other sets of flows.


**Adjustment of Simulation Parameters to HCM Results**

The most important elements to be adjusted when a basic freeway or
multilane highway segment is analyzed are the speed–flow relationship, the
capacity, or both. The speed–flow relationship should be examined as a
function of the given FFS. That FFS should match the field- or HCMestimated value.


**Step-by-Step Recommendations for Applying Alternative Tools**

This section provides recommendations specifically for freeway and
multilane highway segments (general guidance on selecting and applying
simulation packages is provided in Chapter 6, HCM and Alternative
Analysis Tools). To apply an alternative tool to the analysis of basic freeway
and multilane highway segments, the following steps should be taken:

1. Determine whether the chosen tool can provide density and capacity
for a basic freeway or multilane highway segment and the approach
used to obtain those values. Once the analyst is satisfied that density
and capacity can be obtained and that values compatible with those of
the HCM can also be obtained, proceed with the analysis.

2. Determine the FFS of the study site, either from field data or by
estimating it according to this chapter’s methodology.


3. Enter all available geometric and traffic characteristics into the
simulation package and install virtual detectors along the study
segment, if necessary, to obtain speeds and flows.

4. By loading the study network over capacity, obtain the maximum
throughput and compare it with the HCM estimate. Calibrate the
simulation package by modifying parameters related to the minimum
time headway so that the capacity obtained by the simulator closely
matches the HCM estimate. Estimate the number of runs required for
a statistically valid comparison.

5. If the analysis requires evaluating various demand conditions for the
segment, plot the simulator’s speed–flow curve and compare it with
the HCM relationship. Attempt to calibrate the simulation package by
modifying parameters related to driver behavior, such as the
distribution of driver types. Calibration of the simulation to match the
HCM speed–flow relationship may not be possible. In that case, the
results should be viewed with caution in terms of their compatibility
with the HCM methods.


**Sample Calculations Illustrating Alternative Tool Applications**

Chapter 26, Freeway and Highway Segments: Supplemental, in Volume 4
of the HCM, provides two supplemental problems that examine situations
beyond the scope of this chapter’s methodology by using a typical
microsimulation-based tool. Both problems analyze a six-lane freeway
segment in a growing urban area. The first supplemental problem evaluates
the facility when an HOV lane is added, and the second problem analyzes
operations with an incident within the segment.


## **6. REFERENCES**

1. Wang, Y., X. Liu, N. Rouphail, B. Schroeder, Y. Yin, and L. Bloomberg.
_NCHRP Web-Only Document 191: Analysis of Managed Lanes on_
_Freeway Facilities._ Transportation Research Board of the National
Academies, Washington, D.C., Aug. 2012.
http://www.trb.org/Publications/Blurbs/168255.aspx

2. Liu, X., B. J. Schroeder, T. Thomson, Y. Wang, N. M. Rouphail, and Y.
Yin. Analysis of Operational Interactions Between Freeway Managed
Lanes and Parallel, General Purpose Lanes. In _Transportation_
_Research Record: Journal of the Transportation Research Board, No._
_2262_, Transportation Research Board of the National Academies,
Washington, D.C., 2011, pp. 62–73.

3. Schroeder, B. J., S. Aghdashi, N. M. Rouphail, X. C. Liu, and Y. Wang.
Deterministic Approach to Managed Lane Analysis on Freeways in
Context of _Highway Capacity Manual_ . In _Transportation Research_
_Record: Journal of the Transportation Research Board, No. 2286_,
Transportation Research Board of the National Academies, Washington,
D.C., 2012, pp. 122–132.

4. Schoen, J. A., A. May, W. Reilly, and T. Urbanik. _Speed–Flow_
_Relationships for Basic Freeway Sections._ Final Report, NCHRP
Project 03-45. JHK & Associates, Tucson, Ariz., May 1995.

5. Roess, R. _Re-Calibration of the 75-mi/h Speed–Flow Curve and the_
_FFS Prediction Algorithm for HCM 2010_ . Research Memorandum,
NCHRP Project 3-92. Polytechnic Institute of New York University,
Brooklyn, Jan. 2009.

6. Reilly, W., D. Harwood, J. Schoen, and M. Holling. _Capacity and LOS_
_Procedures for Rural and Urban Multilane Highways._ Final Report,
NCHRP Project 3-33. JHK & Associates, Tucson, Ariz., May 1990.


7. Aghdashi S., N. M. Rouphail, A. Hajbabaie, and B. J. Schroeder.
Generic Speed–Flow Models for Basic Freeway Segments on General
Purpose and Managed Lanes. In _Transportation Research Record:_
_Journal of the Transportation Research Board, No. 2483_,
Transportation Research Board of the National Academies, Washington,
D.C., 2015, pp. 102–110.

8. _Highway Functional Classification Concepts, Criteria and_
_Procedures, 2013 Edition_ . Federal Highway Administration,
Washington, D.C., 2013.

9. Basic Freeway Sections. In _Special Report 209: Highway Capacity_
_Manual,_ Chapter 3, Transportation Research Board, National Research
Council, Washington, D.C., 1994.

10. Urbanik, T., II, W. Hinshaw, and K. Barnes. Evaluation of High-Volume
Urban Texas Freeways. In _Transportation Research Record 1320_,
Transportation Research Board, National Research Council,
Washington, D.C., 1991, pp. 110–118.

11. Banks, J. H. Flow Processes at a Freeway Bottleneck. In
_Transportation Research Record 1287_, Transportation Research Board,
National Research Council, Washington, D.C., 1990, pp. 20–28.

12. Hall, F. L., and L. M. Hall. Capacity and Speed–Flow Analysis of the
Queen Elizabeth Way in Ontario. In _Transportation Research Record_
_1287_, Transportation Research Board, National Research Council,
Washington, D.C., 1990, pp. 108–118.

13. Hall, F. L., and K. Agyemang-Duah. Freeway Capacity Drop and the
Definition of Capacity. In _Transportation Research Record 1320_,
Transportation Research Board, National Research Council,
Washington, D.C., 1991, pp. 91–98.

14. Chin, H. C., and A. D. May. Examination of the Speed–Flow
Relationship at the Caldecott Tunnel. In _Transportation Research_
_Record 1320_, Transportation Research Board, National Research
Council, Washington, D.C., 1991, pp. 75–82.

15. Banks, J. H. _Evaluation of the Two-Capacity Phenomenon as a Basis_
_for Ramp Metering._ Final Report. San Diego State University, San
Diego, Calif., 1991.


16. Schroeder, B. J., C. M. Cunningham, D. J. Findley, J. E. Hummer, and R.
S. Foyle. _Manual of Transportation Engineering Studies,_ 2nd ed.
Institute of Transportation Engineers, Washington, D.C., 2010.

17. Webster, N., and L. Elefteriadou. A Simulation Study of Truck
Passenger Car Equivalents (PCE) on Basic Freeway Segments.
_Transportation Research Part B,_ Vol. 33, No. 5, 1999, pp. 323–336.

18. Zegeer, J. D., M. A. Vandehey, M. Blogg, K. Nguyen, and M. Ereti.
_NCHRP Report 599: Default Values for Highway Capacity and Level_
_of Service Analyses._ Transportation Research Board of the National
Academies, Washington, D.C., 2008.

19. Dowling, R., G. List, B. Yang, E. Witzke, and A. Flannery. _NCFRP_
_Report 31: Incorporating Truck Analysis into the_ Highway Capacity
Manual. Transportation Research Board of the National Academies,
Washington, D.C., 2014.

20. Washburn, S. S., and S. Ozkul. _Heavy Vehicle Effects on Florida_
_Freeways and Multilane Highways_ . Report TRC-FDOT-93817-2013.
Florida Department of Transportation, Tallahassee, Oct. 2013.

21. Ozkul, S., and S. S. Washburn. Updated Commercial Truck Speed
Versus Distance–Grade Curves for the _Highway Capacity Manual._ In
_Transportation Research Record: Journal of the Transportation_
_Research Board_, _No. 2483_, Transportation Research Board of the
National Academies, Washington, D.C. 2015, pp. 91–101.

22. Landis, B. W., V. R. Vattikuti, and M. T. Brannick. Real-Time Human
Perceptions: Toward a Bicycle Level of Service. In _Transportation_
_Research Record 1578,_ Transportation Research Board, National
Research Council, Washington, D.C., 1997, pp. 119–126.

23. _Highway Performance Monitoring System Field Manual_, Chapter 4.
Federal Highway Administration, Washington, D.C., May 2005.


