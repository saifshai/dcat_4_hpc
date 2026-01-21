#

#

# **A Lightweight Data Mesh for HPC-Based Computational Science**

### Centralized Metadata for Decentralized Lab-Owned Data Products

**Proposal**

**Date: 05/01/2025**

**Prepared for:**

_AAFC - ACGEO-AER_

**Prepared by:**

_uOttawa SEG Capstone Group_

## **1\. Executive Summary**

Modern computational research labs generate large volumes of data on shared HPC infrastructure. While individual labs retain ownership of their outputs, the result is often fragmented storage, limited discoverability, and duplicated computational effort across teams. This proposal explores a lightweight data mesh architecture localized to HPC environments, where lab groups own and curate their data products while a centralized metadata layer enables discovery, governance, and reuse without centralizing the data itself. The project focuses on architecture, metadata design, and governance concepts, drawing inspiration from industry data mesh patterns (e.g., Google Cloud Platform's Dataplex) and from ESGF (Earth System Grid Federation) as a proven scientific example of metadata-driven federation over distributed data. The goal is to design a system that improves discoverability, reproducibility, and cross-lab collaboration while remaining compatible with HPC constraints and future federation efforts.

## **2\. Context and Scope**

### **2.1 Context**

The target environment is computational research labs operating on shared HPC clusters characterized by file-based storage systems, long-running workflows, and multi-team or multi-lab organizational structures. Data is typically generated through simulations, models, or large-scale analyses, and is governed by lab-specific ownership and domain expertise. These datasets are often long-lived and may need to be reused years later, with access controls enforced primarily through existing filesystem and group permission models.

### **2.2 Scope**

This proposal is in scope for computational research outputs and derived datasets that are treated as lab-owned "data products" stored in separate directories, supported by a centralized metadata layer that enables discovery, lineage, and lightweight governance. It also remains conceptually aligned with modern data management architectures while preserving HPC-native realities. Out of scope are real-time experiment control, centralized data warehouses or lakes, replacement of HPC schedulers/filesystems/workflows, and the definition of organization-wide enforcement or policy mechanisms beyond existing access controls.

## **3\. Problem Statement**

In many HPC research environments, data is distributed across lab directories without a unified discovery mechanism, and knowledge of existing datasets relies heavily on personal communication and institutional memory. Provenance and reproducibility information is often scattered across logs, scripts, and ad hoc documentation. Centralizing all data into a single system is typically impractical due to scale, ownership boundaries, and performance and governance constraints. As a result, similar computations are repeated unnecessarily, valuable datasets remain underutilized, and reproducing past results becomes increasingly difficult over time.

## **4\. Proposed Approach**

We propose a metadata-centric data mesh tailored to HPC environments, based on decentralized data ownership with centralized metadata coordination. Each lab owns, stores, and publishes its data products locally, while a shared coordination layer collects and indexes metadata, enables global discovery across labs, and provides visibility into provenance and relationships between datasets. This approach avoids moving or duplicating large datasets, but still enables collaboration, reuse, and more reliable reproducibility.
![image info](./images/con_model.png)

## **5\. Design Principles**

The system is guided by the following principles: (1) Lab ownership, where each lab remains responsible for the scientific validity and lifecycle of its data products; (2) minimized data movement, where only metadata is centralized and data remains in place; (3) immutability and versioning, where published data products are treated as immutable, versioned entities; (4) HPC-native design, compatible with file-based storage, batch workflows, and restricted network environments; (5) low coupling, so labs can operate independently without runtime dependencies on each other; (6) standards awareness, aligning with scientific federation patterns without requiring them; and (7) implementation flexibility, specifying architectural intent rather than prescribing specific tooling or platforms.

## **6\. Conceptual Architecture**

The architecture is intentionally abstract and role-based. Lab groups act as Data Product Owners who define, publish, and maintain data products. A Metadata Coordination Layer aggregates metadata, supports discovery, and captures lineage. Consumers include other labs, researchers, and downstream workflows that reuse published datasets. Core abstractions include the Data Product (a documented, versioned research output), the Publication Boundary (the point at which a dataset becomes discoverable and reusable), the Metadata Contract (a minimal schema describing semantics, provenance, and ownership), and Federation (a shared index of metadata rather than shared storage).

## **7\. Relationship to Existing Models**

From an industry perspective, the proposal follows data mesh concepts at a conceptual level: domains map to lab groups, "data as a product" maps to published research outputs, federated governance maps to shared metadata standards, and self-serve infrastructure corresponds to existing HPC workflows augmented with a publish step. Cloud platforms such as GCP Dataplex demonstrate the feasibility of decentralized ownership paired with coordinated governance. From a scientific perspective, ESGF demonstrates that large-scale data can be served through federated metadata, that data locality can be preserved, and that centralized discovery can enable scalable collaboration.

## **8\. Governance and Responsibilities**

The governance model is intentionally lightweight and appropriate for research environments. Labs own their data products, define metadata and provenance, and decide when data is published. The shared coordination function maintains metadata standards and shared vocabularies, and provides discovery and visibility into available data products, but does not enforce access control. Access control remains the responsibility of existing filesystem permissions and group-based authorization.

## **9\. Evaluation Criteria**

Success can be assessed through qualitative and quantitative signals including reductions in duplicated computational work, improved ability to discover existing datasets, increased cross-lab reuse of published outputs, the percentage of datasets with documented provenance, and the ease with which new labs can onboard into the system and publish discoverable data products.

## **10\. Conclusion**

This proposal presents a practical, research-oriented exploration of adapting data mesh principles to HPC environments. By centralizing only metadata and preserving decentralized data ownership, the approach respects lab autonomy and HPC constraints while improving discoverability, reproducibility, and collaboration.

##

##

##

## **11\. Appendix A: Key Concepts and Definitions**

### **11.1. Data Mesh**

Data mesh is a modern data management architecture that shifts data ownership and responsibility from a centralized team to the domain teams that produce and understand the data. Rather than collecting all data into a single system, it treats data as a set of independently owned, well-defined data products maintained by the teams closest to the data's scientific or operational context. Core characteristics include domain-oriented ownership, data treated as a product with clear ownership and defined semantics, federated governance through shared standards with decentralized curation, and self-serve infrastructure that enables publishing and maintenance through common conventions and tools. Data mesh is considered "modern" because it reflects environments where data volume and diversity make centralization difficult, domain expertise is essential for interpretation, and autonomy improves organizational outcomes.

### **11.2. Federation**

Federation is an implementation pattern that enables decentralized data serving with centralized discovery and coordination. In a federated system, data remains physically stored and controlled by its owner, a shared layer aggregates metadata, and consumers can discover and access data without needing prior knowledge of its location. Key characteristics include decentralized storage and control, centralized metadata and discovery, loose coupling between providers, and preserved data locality. Federation is commonly used when data is expensive to move, ownership boundaries must be respected, multiple teams contribute data, and long-term accessibility and provenance are important. ESGF is a well-established scientific example of federation, enabling broad discovery and access to climate data without centralizing storage.

### **11.3. Relationship Between Data Mesh and Federation**

Data mesh and federation describe different layers of the same overall approach. Data mesh is an architectural approach that defines how data ownership and responsibility are organized, while federation is an implementation pattern that enables decentralized data to be discoverable and reusable through centralized coordination. In this proposal, data mesh answers the organizational question by assigning ownership and publication responsibility to lab groups, and federation answers the technical coordination question by using a centralized metadata layer to index and relate lab-owned datasets.

.
