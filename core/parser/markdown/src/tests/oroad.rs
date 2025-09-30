pub const OROAD_0: &str = r#"
# OROAD-0: The Attempt
- **Authors:** [Liam Monninger](liam@ramate.io)
- **Contents:**
    - **[Summary](#summary)**
    - **[Roadmap](#roadmap)**
    - **[Agreeing](#agreeing)**
    - **[Dissenting](#dissenting)**
    - **[Appendix](#appendix)**

## Summary
**OROAD-0** is the foundational roadmap for OAC proposed in response to [OPROC-0: Decentralized Consequence](../../../oproc/oera-000-000-000-dulan/oproc-000-000-000/README.md). **OROAD-0** seeks to develop and validate a series of foundational papers, render a series of implementations from these papers, and the output applications demonstrating the utility of these implementations. In the end, **OROAD-0** describes the series efforts which will be used to determine whether OAC is worth pursuing.

The foundational papers anticipated by **OROAD-0** are:
- ****OART-1: BFA**:** describes a class of sampling protocols which accept Byzantine minority decisions with some non-zero probability; additionally formalizes the expected value of said decisions arguing for the ability for them to be rendered irrational. BFA are still deterministic and final.
- ****OART-2: Collaborative Transaction Routing**:** describes a class of sortition-based transaction broadcast protocols. These allow incentivization to be shifted out of native token and into discretionary "super" protocols.
- ****OART-3: RIS-STM**:** describes a generalization of [Block-STM](https://arxiv.org/abs/2203.06871) which plays forward best-case latency.

The foundational implementations anticipated by **OROAD-0** are:
- **[`gwrdfa`](https://github.com/ramate-io/gwrdfa):** an implementation of ****OART-1: BFA**** protocol substack. This forms the basis for high-throughput and large footprint OAC applications.
- **[`srcavei`](https://github.com/ramate-io/srcavei):** an implementation of the ****OART-2: Collaborative Transaction Routing**** substack. This forms the basis for incentivization—which would no longer be strictly coin-based.
- **[`fuste`](https://github.com/ramate-io/fuste):** a RISC-V VM with a set of adapters tailored to DLT—particularly plugging into the stack above. This is also critical to throughput and large footprint.
- **[`zhiye`](https://github.com/ramate-io/zhiye):** implementation of ****OART-3: RIS-STM****. This takes advantage of some properties of BFA to greatly reduce best-case latency.

All of these implementations are part of [Ramate LLC's](https://www.ramate.io) [`robles`](https://github.com/ramate-io/robles) stack.

> [!TIP]
> **[[Liam Monninger]](liam@ramate.io)**
>
> **OROAD-0** speculates much more into specific implementations and applications than I would expect later roadmaps to do. We justify this on the basis of the concept needing to initially validate itself and so needing to demonstrate end-goal utility.
>
> After **OROAD-0** our intent is to shift the planning done by OAC to the more conceptually-focused--almost taking the stance of a journal or academic institution.

## Roadmap
> [!WARNING]
> Ensure **All leads** contains list of all leads from milestones below.
>
> **[AI Prompt]**
>
> Help contributors to ensure the above.

- **All leads:** [Liam Monninger](liam@ramate.io)
- **Contents:**
    - **[T1](#t1-push-towards-validation):** Push Towards Validation
    - **[T2](#t2-validation-and-accepting-contributions):** Validation and Accepting Contributions
    - **[T3](#t3-continued-validation-and-fuste-mvp):** Continued Validation and Fuste MVP
    - **[T4](#t4-exotic-execution):** Exotic Execution
    - **[T5](#t5-dlt-push):** DLT Push
    - **[T6](#t6-killer-apps-phase-1-traditional-l1):** Killer Apps Phase 1: Traditional L1
    - **[T7](#t7-killer-apps-phase-2-content-sharing):** Killer Apps Phase 2: Content Sharing
    - **[T8](#t8-killer-apps-phase-3-content-sharing-continued):** Killer Apps Phase 3: Content Sharing Continued
    - **[T9](#t9-an-interlude)**: An Interlude

### T1: Push Towards Validation
> [!IMPORTANT]
> **T1** focuses on readying OAC for validation.

- **Starts:** T1 + 0 months
- **Depends-on:** $\emptyset$
- **Ends:** T1 + 1 month
- **Contents:**
    - **[T1.1](#t11-complete-draft-of-oart-1-bfa)**: Complete draft of **OART-1: BFA**
    - **[T1.2](#t12-complete-draft-of-oart-2-collaborative-transaction-routing)**: Complete draft of **OART-2: Collaborative Transaction Routing**
    - **[T1.3](#t13-begin-gwrdfa-implementation)**: Begin [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation
    - **[T1.4](#t14-begin-srcavei-implementation)**: Begin [`srcavei`](https://github.com/ramate-io/srcavei) implementation
    - **[T1.5](#t15-begin-fuste-implementation-as-lesser-priority-to-t13-and-t14)**: Begin [`fuste`](https://github.com/ramate-io/fuste) implementation

**T1** features a push towards rendering content which will the initial validation of Ordered Atomic Collaboration (OAC).

**T1** adopts the following sub-roadmaps:

- **[OROAD-5](/oroad/oera-000-000-000-dulan/oroad-000-000-005/README.md)**: Week 0

> [!NOTE]
> **[[Liam Monninger](mailto:liam@ramate.io)]**
>
> **[OROAD-5](/oroad/oera-000-000-000-dulan/oroad-000-000-005/README.md)** makes provisions for lots of utility development that facilitates **[T1](#t1-push-towards-validation)** and other targets. We do not underscore this as a separate point, because the advancement of these utilities is a degree of freedom below the concerns of **OROAD-0**.

**T1** seeks to accomplish the following itemized objectives:

#### T1.1: Complete draft of **OART-1: BFA**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

A complete draft of **OART-1: BFA** is essential for beginning work on [`gwrdfa`](https://github.com/ramate-io/gwrdfa).

> [!NOTE]
> **[[Liam Monninger](mailto:liam@ramate.io)]
>
> This will most likely also be paired with ROSPEC for the [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation. However, such is not directly the concern of OAC.

#### T1.2: Complete draft of **OART-2: Collaborative Transaction Routing**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

A complete draft of **OART-2: Collaborative Transaction Routing** is essential for beginning work on [`srcavei`](https://github.com/ramate-io/srcavei).

> [!NOTE]
> **[[Liam Monninger](mailto:liam@ramate.io)]**
>
> This will most likely also be paired with ROSPEC for the [`srcavei`](https://github.com/ramate-io/srcavei) implementation. However, such is not directly the concern of OAC.

#### T1.3: Begin [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

[`gwrdfa`](https://github.com/ramate-io/gwrdfa) is an implementation of **OART-1: BFA** which will allow us to build apps and demonstrate the utility of the protocol.

#### T1.4: Begin [`srcavei`](https://github.com/ramate-io/srcavei) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

[`srcavei`](https://github.com/ramate-io/srcavei) is an implementation of **OART-2: Collaborative Transaction Routing** which will allow us to build apps and demonstrate the utility of the protocol.

> [!NOTE]
> **[[Liam Monninger](mailto:liam@ramate.io)]**
>
> `srcavei` is however also the easiest component to drop and still have a usable application.

#### T1.5: Begin [`fuste`](https://github.com/ramate-io/fuste) implementation as lesser priority to [T1.3](#t13-begin-gwrdfa-implementation) and [T1.4](#t14-begin-srcavei-implementation)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Beginning [`fuste`](https://github.com/ramate-io/fuste) earlier allows for us to potentially have a viable programmable VM earlier--greatly easing killer app attempts.

### T2: Validation and Accepting Contributions
> [!IMPORTANT]
> **T2** focuses on beginning validation of OAC and adding contributors.

- **Starts:** T1 + 1 month
- **Depends-on:** [T1](#t1-push-towards-validation)
- **Ends:** T2 + 1 month
- **Contents:**
    - **[T2.1](#t21-share-and-gather-feedback-on-oart-1-bfa)**: Share and gather feedback on **OART-1: BFA**
    - **[T2.2](#t22-share-and-gather-feedback-on-oart-2-collaborative-transaction-routing)**: Share and gather feedback on **OART-2: Collaborative Transaction Routing**
    - **[T2.3](#t23-implement-and-document-proposal-standards-contributor-guidelines-and-implementation-governance)**: Implement and document proposal standards, contributor guidelines, and implementation governance
    - **[T2.4](#t24-complete-gwrdfa-reference-implementation)**: Complete [`gwrdfa`](https://github.com/ramate-io/gwrdfa) reference implementation
    - **[T2.5](#t25-complete-srcavei-reference-implementation)**: Complete [`srcavei`](https://github.com/ramate-io/srcavei) reference implementation
    - **[T2.6](#t26-continue-development-of-fuste-as-a-lower-priority-task)**: Continue development of [`fuste`](https://github.com/ramate-io/fuste) as a lower priority task
    -**[T2.7](#t27-develop-and-document-strategy-to-attract-contributors)**: Develop and document strategy to attract contributors

**T2** focuses on validating the initial content and establishing contribution frameworks for Ordered Atomic Collaboration (OAC) and [`robles`](https://github.com/ramate-io/robles).

**T2** seeks to accomplish the following itemized objectives:

#### T2.1: Share and gather feedback on **OART-1: BFA**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

It is critical to gather feedback on **OART-1: BFA**, to ensure viability of the project and its quality.

#### T2.2: Share and gather feedback on **OART-2: Collaborative Transaction Routing**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

It is critical to gather feedback on **OART-2: Collaborative Transaction Routing**, to ensure viability of the project and its quality.

#### T2.3: Implement and document proposal standards, contributor guidelines, and implementation governance
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Clearly establishing contributor guidelines and governance, and ensuring strong developer experience shall facilitate chances at adoption of and contribution to the project.

> [!NOTE]
> **[[Liam Monninger](mailto:liam@ramate.io)]**
>
> At the moment, there is a fairly robust contribution framework availability a lot of this will be fine-tuning.

#### T2.4: Complete [`gwrdfa`](https://github.com/ramate-io/gwrdfa) reference implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Completing the [`gwrdfa`](https://github.com/ramate-io/gwrdfa) reference implementation shall enable the implementation of applications using [`gwrdfa`](https://github.com/ramate-io/gwrdfa).

#### T2.5: Complete [`srcavei`](https://github.com/ramate-io/srcavei) reference implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Completing the [`srcavei`](https://github.com/ramate-io/srcavei) reference implementation shall enable the implementation of applications using [`srcavei`](https://github.com/ramate-io/srcavei).

#### T2.6: Continue development of [`fuste`](https://github.com/ramate-io/fuste) as a lower priority task
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Continued progression of [`fuste`](https://github.com/ramate-io/fuste) again supports ease of killer apps phases.

#### T2.7: Develop and document strategy to attract contributors
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

It is unlikely contributors will simply flock to the project. We will need to seek out and incentivize participation. This will likely mostly begin amongst close colleagues.

### T3: Continued Validation and [`fuste`](https://github.com/ramate-io/fuste) MVP
> [!IMPORTANT]
> **T3** continues validation and pushes for first proper application built with an OAC implementer's stack.

- **Starts:** T2 + 1 month
- **Depends-on:** [T2](#t2-validation-and-accepting-contributions)
- **Ends:** T3 + 1 month
- **Contents:**
    - **[T3.1](#t31-continue-sharing-and-updating-oart-1-bfa)**: Continue sharing and updating **OART-1: BFA**
    - **[T3.2](#t32-continue-sharing-and-updating-oart-2-collaborative-transaction-routing)**: Continue sharing and updating **OART-2: Collaborative Transaction Routing**
    - **[T3.3](#t33-develop-fuste-mvp)**: Develop [`fuste`](https://github.com/ramate-io/fuste) MVP
    - **[T3.4](#t34-use-fuste-mvp-to-develop-centralized-embedded-database)**: Use [`fuste`](https://github.com/ramate-io/fuste) MVP to develop centralized embedded database

**T3** focuses on continued validation of core concepts and the development of the Fuste MVP as a proof of concept for Ordered Atomic Collaboration (OAC).

**T3** seeks to accomplish the following itemized objectives:

#### T3.1: Continue sharing and updating **OART-1: BFA**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Validating **OART-1: BFA** shall create the initial formal basis for the OAC project.

#### T3.2: Continue sharing and updating **OART-2: Collaborative Transaction Routing**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

While also being necessary for demonstrating alternative incentive structures, validating **OART-2: Collaborative Transaction Routing** establishes a more exotic tone the pursuits in OAC beyond **OART-1: BFA**.

#### T3.3: Develop [`fuste`](https://github.com/ramate-io/fuste) MVP
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

At this point, [`fuste`](https://github.com/ramate-io/fuste) should reach an MVP in order to soon facilitate killer app development.

#### T3.4: Use [`fuste`](https://github.com/ramate-io/fuste) MVP to develop centralized embedded database
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

While finalizing [`fuste`](https://github.com/ramate-io/fuste), producing an embedded database shall help to dogfood the programmability layer and demonstrate its utility.

### T4: Exotic Execution
> [!IMPORTANT]
> **T4** departs from the drive of previous milestones and takes some time to explore exotic distributed execution.

- **Starts:** T3 + 1 month
- **Depends-on:** [T3](#t3-continued-validation-and-fuste-mvp)
- **Ends:** T4 + 1 month
- **Contents:**
    - **[T4.1](#t41-draft-and-share-oart-3-ris-stm)**: Draft and share **OART-3: RIS-STM**
    - **[T4.2](#t42-experiment-with-exotic-execution-models-ideally-with-past-colleagues-and-contributors)**: Experiment with exotic execution models, ideally with past colleagues and contributors
    - **[T4.3](#t43-begin-zhiye-implementation)**: Begin [`zhiye`](https://github.com/ramate-io/zhiye) implementation

**T4** focuses on exploring exotic execution models. It serves as a bit of a break from the driving thrust of **[T1](#t1-push-towards-validation)** to **[T3](#t3-continued-validation-and-fuste-mvp)**.

**T4** seeks to accomplish the following itemized objectives:

#### T4.1: Draft and share **OART-3: RIS-STM**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Drafting and sharing **OART-3: RIS-STM** is critical to producing a correct implementation thereof. Without RIS-STM, the proposed horizontal scaling benefits of **OART-1: BFA** would not be easy to realize in development.

#### T4.2: Experiment with exotic execution models, ideally with past colleagues and contributors
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Many of the currently considered exotic execution models draw from emerging theory in disciplines such as topos theory and quantum computing. This target is an accessory; it is mainly intended as a refresher and an opportunity to take a new look at the project overall--potentially generating some interesting and marketable memos along the way.

#### T4.3: Begin [`zhiye`](https://github.com/ramate-io/zhiye) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

[`zhiye`](https://github.com/ramate-io/zhiye) implements **OART-3: RIS-STM**. Its development alongside the drafting of **OART-3: RIS-STM** should help to facilitate a more comprehensive understanding of tradeoffs and optimizations which can be referenced therein.

### T5: DLT Push
> [!IMPORTANT]
> **T5** seeks to bring up stable implementations of OAC decentralization and prepare to use these to create a traditional DLT.

- **Starts:** T4 + 1 month
- **Depends-on:** [T4](#t4-exotic-execution)
- **Ends:** T5 + 1 month
- **Contents:**
    - **[T5.1](#t51-stabilize-gwrdfa-implementation)**: Stabilize [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation
    - **[T5.2](#t52-stabilize-srcavei-implementation)**: Stabilize [`srcavei`](https://github.com/ramate-io/srcavei) implementation
    - **[T5.3](#t53-seek-out-additional-co-authors-for-oac-foundational-papers-oart-1-bfa-oart-2-collaborative-transaction-routing-and-oart-3-ris-stm)**: Seek out additional co-authors for OAC foundational papers
    - **[T5.4](#t54-prepare-oart-1-bfa-for-conference-submission)**: Prepare **OART-1: BFA** for conference submission
    - **[T5.5](#t55-prepare-oart-2-collaborative-transaction-routing-for-conference-submission)**: Prepare **OART-2: Collaborative Transaction Routing** for conference submission
    - **[T5.6](#t56-begin-search-for-funding-opportunities)**: Begin search for funding opportunities

**T5** focuses on stabilizing core implementations and preparing academic work for broader dissemination within the Ordered Atomic Collaboration (OAC) framework.

**T5** seeks to accomplish the following itemized objectives:

#### T5.1: Stabilize [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Stable [`gwrdfa`](https://github.com/ramate-io/gwrdfa) will form the consensus layer for the DLT.

#### T5.2: Stabilize [`srcavei`](https://github.com/ramate-io/srcavei) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Stable [`srcavei`](https://github.com/ramate-io/srcavei) will form the incentive layer for the DLT.

#### T5.3: Seek out additional co-authors for OAC foundational papers: **OART-1: BFA**, **OART-2: Collaborative Transaction Routing**, and **OART-3: RIS-STM**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

All three foundational papers will likely need additional co-authors in order to achieve high-quality and have a chance at publication in a strong journal.

#### T5.4: Prepare **OART-1: BFA** for conference submission
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Upon completion of **[T5.3](#t53-seek-out-additional-co-authors-for-oac-foundational-papers-oart-1-bfa-oart-2-collaborative-transaction-routing-and-oart-3-ris-stm)**, we should be able to begin shopping conferences for **OART-1: BFA** and modifying it as needed. The sooner we find a conference, the sooner we can consider the downstream implementation stable.

#### T5.5: Prepare **OART-2: Collaborative Transaction Routing** for conference submission
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Upon completion of **[T5.3](#t53-seek-out-additional-co-authors-for-oac-foundational-papers-oart-1-bfa-oart-2-collaborative-transaction-routing-and-oart-3-ris-stm)**, we should be able to begin shopping conferences for **OART-2: Collaborative Transaction Routing** and modifying it as needed. The sooner we find a conference, the sooner we can consider the downstream implementation stable.

#### T5.6: Begin search for funding opportunities
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Beginning the search for funding opportunities early will allow ample consideration of financing options and potential partners. In all likelihood, substantial legal groundwork would need to take place between **[T5](#t5-dlt-push)** and **[T9](#t9-an-interlude)** to position **[Ramate LLC](https://www.ramate.io)** to continue to fund this project.

Additionally, this is also when **[[Liam Monninger's](mailto:liam@ramate.io)]** vested assets from Movement should be available and the possibility of bootstrapping from market sales of these funds can be considered more seriously.

### T6: Killer Apps Phase 1: Traditional L1
> [!IMPORTANT]
> **T6** emphasizes the support of the first killer app built with OAC: a traditional L1 blockchain.

- **Starts:** T5 + 1 month
- **Depends-on:** [T5](#t5-dlt-push)
- **Ends:** T6 + 1 month
- **Contents:**
    - **[T6.1](#t61-design-and-implement-traditional-l1-blockchain-applications-using-oac-principles)**: Design and implement traditional L1 blockchain applications using OAC principles
    - **[T6.2](#t62-integrate-gwrdfa-and-srcavei-into-l1-applications)**: Integrate [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei) into L1 applications
    - **[T6.3](#t63-document-and-share-implementation-patterns-for-oac-based-l1-applications)**: Document and share implementation patterns for OAC-based L1 applications
    - **[T6.4](#t64-begin-gathering-feedback-from-the-broader-blockchain-community)**: Begin gathering feedback from the broader blockchain community

**T6** focuses on developing traditional L1 blockchain applications within the Ordered Atomic Collaboration (OAC) framework.

**T6** seeks to accomplish the following itemized objectives:

#### T6.1: Design and implement traditional L1 blockchain applications using OAC principles
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Designing and implementing a traditional L1 blockchain or applications that would usually exist in a blockchain context is a natural application of both OAC technologies and **[[Liam Monninger's](mailto:liam@ramate.io)]** expertise. Further, it has easy-to-assess marketability and prospective communities would be well known.

#### T6.2: Integrate [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei) into L1 applications
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Building with [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei) is the best way to showcase OAC technologies use for an L1.

#### T6.3: Document and share implementation patterns for OAC-based L1 applications
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Documenting and sharing implementations patterns for OAC-based L1 applications will serve to generate the first practical guide for using the OAC stack.

#### T6.4: Begin gathering feedback from the broader blockchain community
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Gauging interest of the blockchain community may identify a simple path for project continuity.

### T7: Killer Apps Phase 2: Content Sharing
> [!IMPORTANT]
> **T7** emphasizes the support of a content sharing application built with OAC implementations.

- **Starts:** T6 + 1 month
- **Depends-on:** [T6](#t6-killer-apps-phase-1-traditional-l1)
- **Ends:** T7 + 1 month
- **Contents:**
    - **[T7.1](#t71-begin-building-content-sharing-platform-using-gwrdfa-and-srcavei)**: Begin building collaborative streaming platform using [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei)
    - **[T7.2](#t72-seek-collaborators-from-traditional-streaming-and-p2p-content-sharing-communities)**: Seek collaborators from traditional streaming and p2p content sharing communities
    - **[T7.3](#t73-guide-and-support-l1-killer-apps-demonstration-and-deployment)**: Guide and support L1 killer apps demonstration and deployment

**T7** focuses on developing content sharing applications and expanding the OAC ecosystem through partnerships and demonstrations.

**T7** seeks to accomplish the following itemized objectives:

#### T7.1: Begin building content sharing platform using [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

A content sharing app named "Thro" has been referenced in [RPRE-0](https://github.com/ramate-io/ramate/blob/main/rpre/rera-000-000-000-dulan/rpre-000-000-000/README.md) as a consumer application which may (a) generate revenue for Ramate and/or (b) demonstrate utility of the OAC stack. Either outcome would provide positive signal to continue the OAC project.

#### T7.2: Seek collaborators from traditional streaming and p2p content sharing communities
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Gather collaborators from traditional streaming and p2p content sharing communities may expand the project's reach and improve the quality of the output.

#### T7.3: Guide and support L1 killer apps demonstration and deployment
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Continued support of L1 killer apps may continue to present opportunities for project funding or [Ramate LLC](https://www.ramate.io) revenue models which could sustain OAC.

### T8: Killer Apps Phase 3: Content Sharing Continued
> [!IMPORTANT]
> **T8** emphasizes completion of an MVP of the content sharing application and ultimately seeks to determine whether the project is worth continuing to pursue in **[T9](#t9-an-interlude)**

- **Starts:** T7 + 1 month
- **Depends-on:** [T7](#t7-killer-apps-phase-2-content-sharing)
- **Ends:** T8 + 1 month
- **Contents:**
    - **[T8.1](#t81-push-for-mvp-of-content-sharing-mobile-application)**: Guide and support L1 blockchain applications demonstration
    - **[T8.2](#t82-guide-and-support-l1-blockchain-applications-demonstration)**: Guide and support Collaborative Streaming platform demonstration
    - **[T8.3](#t83-research-and-experiment-with-swarm-coordination-mechanisms)**: Research and experiment with swarm coordination mechanisms

**T8** focuses on finalizing demonstrations, exploring swarm coordination, and pushing for broader academic recognition of Ordered Atomic Collaboration (OAC).

**T8** seeks to accomplish the following itemized objectives:

#### T8.1: Push for MVP of content sharing mobile application
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Ideally, the development of a "Thro" MVP presents a marketable asset as presented in [RPRE-0](https://github.com/ramate-io/ramate/blob/main/rpre/rera-000-000-000-dulan/rpre-000-000-000/README.md). Placing this MVP in [T8](#t8-killer-apps-phase-3-content-sharing-continued) does not provide much time to reach sufficient market penetration. However, positive adoption should be sufficient to either (a) make it reasonable to continue to bootstrap for several months longer or (b) feasible to seek favorable outside investment.

Alternatively, if "Thro's" financials are insufficient for either of the above, its optics may still help present OAC itself as a marketable asset to other application developers.

#### T8.2: Guide and support L1 blockchain applications demonstration
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

While "Thro" remains the priority, L1 blockchain support may once again continue to present opportunities for OAC.

#### T8.3: Research and experiment with swarm coordination mechanisms
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Allowing a period to research and experiment with swarm coordination both provides a lighter target against which "Thro" development can b safely prioritized and potentially generates marketable material describing OAC's long-term trajectory. Users may like to say "I am using a content sharing app which is funding AI safety."

### T9: An Interlude
> [!IMPORTANT]
> **T9** is a milestone conditional on a positive decision from [T8](#t8-killer-apps-phase-3-content-sharing-continued). If [T8](#t8-killer-apps-phase-3-content-sharing-continued) finds reason to continue the OAC project, OAC will prioritize reorganization of OAC and implementers while progressing swarm coordination research towards a swarm coordination application.

- **Starts:** T8 + 1 month
- **Depends-on:** [T8](#t8-killer-apps-phase-3-content-sharing-continued)
- **Ends:** T9 + 1 month
- **Contents:**
    - **[T9.1](#t91-update-the-governance-of-oac-for-greater-decentralization)**: Update the governance of OAC for greater decentralization
    - **[T9.2](#t92-make-decision-on-bootstrapping-viability)**: Make decision on bootstrapping viability
    - **[T9.3](#t93-final-push-for-academic-recognition-of-oart-1-bfa)**: Final push for academic recognition of **OART-1: BFA**
    - **[T9.4](#t94-final-push-for-academic-recognition-of-oart-2-collaborative-transaction-routing)**: Final push for academic recognition of **OART-2: Collaborative Transaction Routing**
    - **[T9.5](#t95-final-push-for-academic-recognition-of-oart-3-ris-stm)**: Final push for academic recognition of **OART-3: RIS-STM**

> [!TIP]
> **[[Liam Monninger](liam@ramate.io)]**
>
> **T9** was originally added to **OROAD-0** to help itemize the organizational intents of OAC. The scope has since been altered.

**T9** is a milestone which accounts for the differing success and funding possibilities for the OAC project. Namely, these are:

1. The project continues, but as essentially a hobby project seeking to slowly groundswell with open source contribution. At the end of **[T9](#t9-an-interlude)** this likely stands as a portfolio project for **[[Liam Monninger](mailto:liam@ramate.io)]**.
2. The project finds a means to bootstrap but **[[Liam Monninger](mailto:liam@ramate.io)]** remains the sole full-time contributor.
3. The project finds a means to bootstrap and is able to hire additional full-time contributors.
4. The project takes begins to take on investment or depends on investment into Ramate which has obligations.

In the face of all of these possibilities, ensuring a relative stasis for the governance and the core conceptual basis of OAC is regarded as paramount.

**T9** seeks to accomplish the following itemized objectives:

#### T9.1: Update the governance of OAC for greater decentralization
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

We intend to push for greater decentralization of OAC governance. OAC should be organization whose development is guided openly and transparently by many parties. Just as our technology derives consequence by participation, so to should the organization developing said technology.

In the very least, this update of governance should include moving OAC out from under [Ramate LLC's](https://www.ramate.io) governance if such has not already occurred.

#### T9.2: Make decision on bootstrapping viability
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

At this point, bootstrapping viability needs to be decided upon. If it is not viable, but it is otherwise evident that the OAC project should continue full-time, then outside investment should be pursued throughout the month.

#### T9.3: Final push for academic recognition of **OART-1: BFA**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

In whichever case, achieving academic acceptance of OAC protocols will help to ensure the chance of long-term value for the project.

#### T9.4: Final push for academic recognition of **OART-2: Collaborative Transaction Routing**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

In whichever case, achieving academic acceptance of OAC protocols will help to ensure the chance of long-term value for the project.

#### T9.5: Final push for academic recognition of **OART-3: RIS-STM**
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

In whichever case, achieving academic acceptance of OAC protocols will help to ensure the chance of long-term value for the project.

## Agreeing
$\emptyset$

## Dissenting
$\emptyset$

## Appendix
$\emptyset$

<!--OAC FOOTER: DO NOT REMOVE THIS LINE-->
---

<div align="center">
  <a href="https://github.com/ramate-io/oac">
    <picture>
      <source srcset="/assets/oac-inverted-transparent.png" media="(prefers-color-scheme: dark)">
      <img height="24" src="/assets/oac-transparent.png" alt="OAC"/>
    </picture>
  </a>
  <br/>
  <sub>
    <b>Ordered Atomic Collaboration (OAC)</b>
    <br/>
    &copy; 2025 <a href="https://github.com/ramate-io/oac">ramate-io/oac</a>
    <br/>
    <a href="https://github.com/ramate-io/oac/blob/main/LICENSE">MIT License</a>
    <br/>
    <a href="https://www.ramate.io">ramate.io</a>
  </sub>
</div>
"#;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::MarkdownParseError;
	use crate::RoadmapParser;

	#[test]
	fn test_parse_oroad_0() -> Result<(), MarkdownParseError> {
		let parser = RoadmapParser::new();
		let tasks = parser.parse_tasks(OROAD_0)?;

		// Should parse 9 tasks (T1 through T9)
		assert_eq!(tasks.len(), 9);

		// Test T1: Push Towards Validation
		let t1 = &tasks[0];
		assert_eq!(t1.id().value(), 1);
		assert_eq!(t1.title().text, "Push Towards Validation");
		assert!(t1.depends_on().is_empty()); // Depends on $\emptyset$
		assert_eq!(t1.subtasks().len(), 5); // T1.1 through T1.5
		assert!(t1.summary().text.contains("**T1** focuses on readying OAC for validation"));

		// Test T2: Validation and Accepting Contributions
		let t2 = &tasks[1];
		assert_eq!(t2.id().value(), 2);
		assert_eq!(t2.title().text, "Validation and Accepting Contributions");
		assert_eq!(t2.depends_on().len(), 1);
		assert!(t2.depends_on().contains(&roadline_util::task::Id::new(1))); // Depends on T1
		assert_eq!(t2.subtasks().len(), 7); // T2.1 through T2.6 + subsection
		assert!(t2.summary().text.contains("**T2** focuses on beginning validation of OAC"));

		// Test T3: Continued Validation and Fuste MVP
		let t3 = &tasks[2];
		assert_eq!(t3.id().value(), 3);
		assert_eq!(
			t3.title().text,
			"Continued Validation and [`fuste`](https://github.com/ramate-io/fuste) MVP"
		);
		assert_eq!(t3.depends_on().len(), 1);
		assert!(t3.depends_on().contains(&roadline_util::task::Id::new(2))); // Depends on T2
		assert_eq!(t3.subtasks().len(), 4); // T3.1 through T3.4
		assert!(t3
			.summary()
			.text
			.contains("**T3** continues validation and pushes for first proper application"));

		// Test T4: Exotic Execution
		let t4 = &tasks[3];
		assert_eq!(t4.id().value(), 4);
		assert_eq!(t4.title().text, "Exotic Execution");
		assert_eq!(t4.depends_on().len(), 1);
		assert!(t4.depends_on().contains(&roadline_util::task::Id::new(3))); // Depends on T3
		assert_eq!(t4.subtasks().len(), 3); // T4.1 through T4.3
		assert!(t4
			.summary()
			.text
			.contains("**T4** departs from the drive of previous milestones"));

		// Test T5: DLT Push
		let t5 = &tasks[4];
		assert_eq!(t5.id().value(), 5);
		assert_eq!(t5.title().text, "DLT Push");
		assert_eq!(t5.depends_on().len(), 1);
		assert!(t5.depends_on().contains(&roadline_util::task::Id::new(4))); // Depends on T4
		assert_eq!(t5.subtasks().len(), 6); // T5.1 through T5.6
		assert!(t5.summary().text.contains("**T5** seeks to bring up stable implementations"));

		// Test T6: Killer Apps Phase 1: Traditional L1
		let t6 = &tasks[5];
		assert_eq!(t6.id().value(), 6);
		assert_eq!(t6.title().text, "Killer Apps Phase 1: Traditional L1");
		assert_eq!(t6.depends_on().len(), 1);
		assert!(t6.depends_on().contains(&roadline_util::task::Id::new(5))); // Depends on T5
		assert_eq!(t6.subtasks().len(), 4); // T6.1 through T6.4
		assert!(t6
			.summary()
			.text
			.contains("**T6** emphasizes the support of the first killer app"));

		// Test T7: Killer Apps Phase 2: Content Sharing
		let t7 = &tasks[6];
		assert_eq!(t7.id().value(), 7);
		assert_eq!(t7.title().text, "Killer Apps Phase 2: Content Sharing");
		assert_eq!(t7.depends_on().len(), 1);
		assert!(t7.depends_on().contains(&roadline_util::task::Id::new(6))); // Depends on T6
		assert_eq!(t7.subtasks().len(), 3); // T7.1 through T7.3
		assert!(t7
			.summary()
			.text
			.contains("**T7** emphasizes the support of a content sharing application"));

		// Test T8: Killer Apps Phase 3: Content Sharing Continued
		let t8 = &tasks[7];
		assert_eq!(t8.id().value(), 8);
		assert_eq!(t8.title().text, "Killer Apps Phase 3: Content Sharing Continued");
		assert_eq!(t8.depends_on().len(), 1);
		assert!(t8.depends_on().contains(&roadline_util::task::Id::new(7))); // Depends on T7
		assert_eq!(t8.subtasks().len(), 3); // T8.1 through T8.3
		assert!(t8.summary().text.contains("**T8** emphasizes completion of an MVP"));

		// Test T9: An Interlude
		let t9 = &tasks[8];
		assert_eq!(t9.id().value(), 9);
		assert_eq!(t9.title().text, "An Interlude");
		assert_eq!(t9.depends_on().len(), 1);
		assert!(t9.depends_on().contains(&roadline_util::task::Id::new(8))); // Depends on T8
		assert_eq!(t9.subtasks().len(), 5); // T9.1 through T9.5
		assert!(t9
			.summary()
			.text
			.contains("**T9** is a milestone conditional on a positive decision"));

		// Test that all tasks have valid ranges
		for (i, task) in tasks.iter().enumerate() {
			let task_id = i + 1;
			// Start should reference the previous task (except T1 which references itself)
			let expected_ref = if task_id == 1 { 1 } else { task_id - 1 };
			assert_eq!(
				usize::from(task.range().start().point_of_reference().0.value()),
				expected_ref
			);

			// End should be a duration (1 month)
			// We can't easily test the exact duration value, but we can ensure it's not zero
			assert!(task.range().end().duration().0.as_secs() > 0);
		}

		// Test that the roadline can be built successfully
		let roadline = parser.parse_and_build(OROAD_0)?;
		let task_count = roadline.tasks().count();
		assert_eq!(task_count, 9);

		Ok(())
	}
}
