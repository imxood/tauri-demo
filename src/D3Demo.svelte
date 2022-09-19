<script lang="ts">
    import * as d3 from "d3";
    import { schemeGnBu } from "d3";
    import { onMount, SvelteComponent } from "svelte";
    let div_ele: HTMLElement;

    let data = [];
    for (let i = 0; i < 100; ++i) {
        data.push({ x: Math.random() * 10, y: Math.random() * 10 });
    }

    let margin = { top: 60, bottom: 60, left: 60, right: 60 };

    let width: number;
    let height: number;

    let xScale = d3.scaleLinear().domain([0, 10]);
    let yScale = d3.scaleLinear().domain([0, 10]);

    onMount(() => {
        redraw();
        window.addEventListener("resize", redraw);
    });

    function redraw() {
        d3.select(div_ele).html(null);
        let rect = d3.select(div_ele).node().getBoundingClientRect();
        width = rect.width - margin.left - margin.right;
        height = rect.height - margin.top - margin.bottom;

        xScale.range([0, width]);
        yScale.range([0, height]);

        const svg = d3
            .select(div_ele)
            .append("svg")
            .attr("width", rect.width)
            .attr("height", rect.height)
            .append("g")
            .attr("transform", `translate(${[margin.left, margin.top]})`);

        // 画 x轴
        svg.append("g")
            .attr("transform", `translate(${[0, height]})`)
            .call(d3.axisBottom(xScale));

        // 画 y轴
        svg.append("g").call(d3.axisLeft(yScale));

        svg.append("g")
            .selectAll("circle")
            .data(data)
            .enter()
            .append("circle")
            .attr("cx", (d) => {
                return xScale(d.x);
            })
            .attr("cy", (d) => {
                return yScale(d.y);
            })
            .attr("r", 7)
            .style("fill", "#ff3e00")
            .style("fill-opacity", "0.5")
            .attr("stroke", "#ff3e00");
    }
</script>

<div class="h-100%" bind:this={div_ele} />
