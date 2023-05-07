Link to dataset: https://snap.stanford.edu/data/roadNet-CA.html

Final Report
            
Author: George Jiang

1. Introduction
In this project, I analyzed a dataset of road networks using the Rust programming language. I created a graph from the dataset and calculated the centrality of each node in the graph. I then output the top 10 nodes with the highest centrality and generate a graph visualization of the network. Overall, I wanted to explore which nodes were the most crucial in road networks of California from SNAP (Stanford Network Analysis Project). The data used is a file containing road network data, named “roadNet-CA.txt”. Each line in the file represents a road connection between two nodes. In other words, the first column indicates the first node and the second column indicates the second node. The dataset contains 1,965,206 nodes and 2,766,607 edges. My program reads in the data from this file and creates a graph with nodes representing the road intersections and edges representing the connections between them. I only use the first 13,000 lines to demonstrate the graph visualization.

2. Methodology
Using the petgraph library in Rust, I was able to construct a directed graph from the dataset. I first generate a DOT file and a TXT file of the graph using the petgraph and the dot crate. Using the “dot” command, the DOT file is converted into a PNG file, which is then used to visualize the actual graph using Graphviz. The resulting image is saved as “graph.png” as shown in Figure 1.

<div align="center">

![Figure 1](https://user-images.githubusercontent.com/132787116/236688410-6d4d7f6e-8ae5-4644-a9d0-19bf81ee8719.png)

*Figure 1. A small portion of the graph generated based on the dataset*

</div>

I then calculated the centrality of each node in the graph using the degree centrality measure, which is calculated by the number of edges to a node divided by the number of possible edges in the graph. I chose this measure because it is a simple and commonly used metric for determining the importance of nodes in a network. The top 100 nodes with the highest centrality are saved to the output file, “output.txt” as shown in Figure 2. This took into account all of the data points, and it wasn’t limited to a smaller part of the dataset like when visualizing.

<div align="center">

![Figure 2](https://user-images.githubusercontent.com/132787116/236688416-1294cedf-26ef-4541-8b39-db07dea9aaba.png)

*Figure 2. The nodes with the highest centrality from biggest to smallest*

</div>

3. Analysis

Based on the entire dataset, node 571924 has the highest centrality and plays the most crucial role in connecting the road networks in California. I find it interesting that the centrality remains the same for nodes 2 to 3 and for 3 to the end of the figure. This tells me that there could be multiple paths between two nodes to take, so the centrality measure may be evenly split for those paths, which would explain the same centrality measure for different nodes. However, to gain a deeper understanding of the nodes' meaning, it would be necessary to consult domain experts familiar with the specifics of the road network or through another possible way. Even though the centrality measure I got appears to be small, in reality, that is to be expected for such a large dataset.

4. Conclusion

In conclusion, I analyzed a dataset of road networks using Rust and calculated the centrality of each node in the network. My analysis identifies the most important nodes in the network, which is useful for real world applications such as transportation planning and infrastructure development. Overall, the analysis conducted provides valuable insights into the centrality of nodes in the California road network, highlighting important areas that warrant further investigation or consideration in transportation planning and network optimization.



















Works Cited

“California Road Network.” SNAP, https://snap.stanford.edu/data/roadNet-CA.html.
Leskovec, Jure, et al. “Community Structure in Large Networks: Natural Cluster Sizes and the Absence of Large Well-Defined Clusters.” ArXiv.org, 8 Oct. 2008, https://arxiv.org/abs/0810.1355. 
“Crate Petgraph.” Petgraph - Rust, https://docs.rs/petgraph/latest/petgraph/. 
“The Rust Programming Language.” How to Write Tests - The Rust Programming Language, https://doc.rust-lang.org/book/ch11-01-writing-tests.html. 
“Crate Dot [−] [Src].” Dot - Rust, https://docs.rs/dot/latest/dot/. 
JaredJared 4, et al. “What's the De-Facto Way of Reading and Writing Files in Rust 1.x?” Stack Overflow, 1 Mar. 1962, https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x. 
“Compute Centrality Scores to Measure the Importance of Entities.” Compute Centrality Scores to Measure the Importance of Entities-ArcGIS Pro | Documentation, https://pro.arcgis.com/en/pro-app/latest/help/data/knowledge/compute-centrality-scores-to-measure-the-importance-of-link-chart-entities.htm. 

