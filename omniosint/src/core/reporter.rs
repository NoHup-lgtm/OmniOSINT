use crate::core::types::Target;
use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub struct HtmlReporter;

impl HtmlReporter {
    pub fn save_report(results: &[Target], path: &str) -> Result<()> {
        let html_content = Self::generate_html(results);
        let mut file = File::create(path)?;
        file.write_all(html_content.as_bytes())?;
        Ok(())
    }

    fn generate_html(results: &[Target]) -> String {
        let mut nodes_js = String::from("[\n");

        if let Some(first) = results.first() {
            nodes_js.push_str(&format!(
                "{{id: 0, label: '{}', group: '{:?}'}},\n",
                first.value, first.kind
            ));
        }

        for (i, target) in results.iter().enumerate().skip(1) {
            let clean_val = target.value.replace("'", "");

            nodes_js.push_str(&format!(
                "{{id: {}, label: '{}', group: '{:?}'}},\n",
                i, clean_val, target.kind
            ));

        }

        nodes_js.push_str("]");

        format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <title>OmniOSINT Report</title>
    <script type="text/javascript" src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
    <style type="text/css">
        body {{ background-color: #111; color: #0f0; font-family: monospace; }}
        #mynetwork {{ width: 100%; height: 800px; border: 1px solid #333; }}
        h1 {{ text-align: center; text-shadow: 0 0 10px #0f0; }}
        .stats {{ text-align: center; margin-bottom: 20px; color: #fff; }}
    </style>
</head>
<body>
    <h1>OmniOSINT Operations Graph</h1>
    <div class="stats">Total Targets Found: {count}</div>
    <div id="mynetwork"></div>
    <script type="text/javascript">
        // Dados vindos do Rust
        var nodes = new vis.DataSet({nodes});
        
        // Lógica de Conexão Automática (JavaScript)
        var edges_arr = [];
        var all_nodes = nodes.get();
        
        all_nodes.forEach(function(nodeA) {{
            all_nodes.forEach(function(nodeB) {{
                if (nodeA.id !== nodeB.id) {{
                    // Regra 1: Subdomínio conecta no Domínio (Ex: api.github.com -> github.com)
                    if (nodeA.label.includes(nodeB.label) && nodeB.group === 'Domain' && nodeA.group === 'Domain') {{
                        // Evita conectar github.com em github.com
                        if (nodeA.label !== nodeB.label) {{
                             edges_arr.push({{from: nodeB.id, to: nodeA.id}});
                        }}
                    }}
                    
                    // Regra 2: Portas e Tech conectam no seu Host
                    // Ex: 'github.com:443' contém 'github.com'
                    if (nodeA.label.includes(nodeB.label) && (nodeA.group === 'OpenPort' || nodeA.group === 'Technology' || nodeA.group === 'Email')) {{
                         edges_arr.push({{from: nodeB.id, to: nodeA.id}});
                    }}
                }}
            }});
        }});

        var edges = new vis.DataSet(edges_arr);

        var container = document.getElementById('mynetwork');
        var data = {{ nodes: nodes, edges: edges }};
        var options = {{
            nodes: {{
                shape: 'dot',
                size: 20,
                font: {{ size: 14, color: '#ffffff' }},
                borderWidth: 2
            }},
            groups: {{
                Domain: {{ color: '#d32f2f', size: 30 }}, 
                OpenPort: {{ color: '#fbc02d', size: 15 }},
                Technology: {{ color: '#7b1fa2', size: 15 }},
                Email: {{ color: '#1976d2', size: 20 }},
                SensitiveFile: {{ color: '#ff6d00', size: 25 }},
            }},
            physics: {{
                stabilization: false,
                barnesHut: {{ gravitationalConstant: -30000, springConstant: 0.04, springLength: 95 }}
            }}
        }};
        var network = new vis.Network(container, data, options);
    </script>
</body>
</html>
        "#,
            count = results.len(),
            nodes = nodes_js
        )
    }
}
