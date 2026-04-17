// examples/with_images.rs
//
// Demonstrates image rendering using publicly available sample images.
// Also uses header/footer, heading numbers, and extra CSS.
// Produces 2+ pages.
//
// Run:  cargo run --example with_images

use pdfsmith::{FooterConfig, HeaderConfig, PdfBuilder};

fn main() {
    env_logger::init();

    let markdown = r##"
# Wildlife Photography Gallery

A curated collection of public-domain wildlife photographs, demonstrating
how `pdfsmith` renders images from URLs inside a PDF.  Images are fetched
by the headless Chrome browser during rendering.

## African Savanna

### Lion

![Lion resting on the savanna](https://images.unsplash.com/photo-1614027164847-1b28cfe1df60?q=80&w=786&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D)

The African lion (*Panthera leo*) is the second-largest living cat after the
tiger.  Males are easily recognised by their mane.  Lions are social cats,
living in groups called **prides** that typically consist of related females,
their cubs, and a small number of adult males.

| Fact          | Detail                         |
|---------------|--------------------------------|
| Scientific name | *Panthera leo*               |
| Weight        | 150–250 kg (males)             |
| Lifespan      | 10–14 years (wild)             |
| Diet          | Carnivore                      |
| Habitat       | Savanna, grassland             |
| Conservation  | Vulnerable (IUCN)             |

### Elephant

![African elephant](https://images.unsplash.com/photo-1557050543-4d5f4e07ef46?q=80&w=1632&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D)

The African bush elephant (*Loxodonta africana*) is the largest living
terrestrial animal.  Bulls can reach 6 tonnes and stand 3.5 metres at the
shoulder.  They are highly intelligent, display complex social behaviours,
and have excellent long-term memory.

Key characteristics:

- Largest brain of any land animal (~5 kg)
- Can live up to 70 years
- Communicate via infrasound over several kilometres
- Matriarchal family groups of 10–15 individuals

## Marine Life

### Sea Turtle

![Green sea turtle swimming](https://images.unsplash.com/photo-1437622368342-7a3d73a34c8f?q=80&w=1528&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D)

The green sea turtle (*Chelonia mydas*) is one of the largest sea turtles
and the only herbivore among the different species.  Named for the green
colour of its cartilage and fat (not its shell), it can weigh up to 315 kg.

### Coral Reef

![Coral reef ecosystem](https://images.unsplash.com/photo-1546026423-cc4642628d2b?q=80&w=1074&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D)

Coral reefs support approximately **25% of all marine species** despite
covering less than 1% of the ocean floor.  They are sometimes called the
"rainforests of the sea."

| Fact             | Detail                           |
|------------------|----------------------------------|
| Total area       | ~284,300 km²                    |
| Species supported| ~1 million                       |
| Growth rate      | 1–3 cm per year (hard coral)    |
| Threat level     | Critical (climate, acidification)|

## Mountain & Forest

### Red Fox

![Red fox in winter](https://images.unsplash.com/photo-1474511320723-9a56873867b5?q=80&w=2072&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D)

The red fox (*Vulpes vulpes*) has the widest geographical range of any
member of the order Carnivora, spanning across the Northern Hemisphere.
They are remarkably adaptable and can thrive in urban environments.

Behavioural notes:

1. **Solitary hunter** — unlike wolves, foxes hunt alone
2. **Omnivorous diet** — rodents, berries, insects, garbage
3. **Excellent hearing** — can locate prey under 60 cm of snow
4. **Cache food** — buries surplus prey for later retrieval

### Bald Eagle

![Bald eagle in flight](https://images.unsplash.com/photo-1667927028600-f0484c9e4615?q=80&w=2080&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D)

The bald eagle (*Haliaeetus leucocephalus*) is the national bird and symbol
of the United States.  With a wingspan of up to 2.3 metres, it is one of
the most impressive raptors in North America.

Conservation success story:

- 1963: Only **417 breeding pairs** left in the lower 48 states
- DDT ban (1972) + Endangered Species Act (1973)
- 2007: Removed from endangered species list
- 2020: Estimated **316,700 individuals** — a remarkable recovery

## Summary

This example demonstrates:

- **Image rendering** from public URLs (Wikipedia Commons)
- Images automatically **scaled to page width** (`max-width: 100%`)
- Mixed content: images + tables + lists + paragraphs
- Header and footer with page numbers
- Heading numbers for clear structure

---

> In every walk with nature, one receives far more than he seeks.  — John Muir
"##;

    let pdf = PdfBuilder::new()
        .title("Wildlife Gallery")
        .display_header_footer(true)
        .header(HeaderConfig {
            center: Some("Wildlife Photography Gallery".into()),
            font_size: Some("9px".into()),
            color: Some("#2e7d32".into()),
            ..Default::default()
        })
        .footer(FooterConfig {
            left: Some("Images: Wikimedia Commons (Public Domain)".into()),
            right: Some(
                r#"Page <span class="pageNumber"></span> of <span class="totalPages"></span>"#
                    .into(),
            ),
            font_size: Some("7px".into()),
            color: Some("#888".into()),
            ..Default::default()
        })
        .heading_numbers(true)
        .extra_css(r#"
            img {
                display: block;
                margin: 16px auto;          /* centred horizontally */
                width: 250px;               /* small fixed size     */
                height: 250px;              /* square                */
                object-fit: cover;          /* crop to fill square   */
                border-radius: 8px;
                box-shadow: 0 2px 10px rgba(0,0,0,0.18);
            }
            h1 { color: #2e7d32; }
        "#)
        .page_load_wait_secs(5)   // extra wait for images to download
        .from_markdown(markdown)
        .expect("Failed to generate PDF");

    std::fs::write("examples/output_pdfs/with_images.pdf", &pdf).expect("Failed to write PDF");
    println!("PDF saved to examples/output_pdfs/with_images.pdf ({} bytes)", pdf.len());
}
