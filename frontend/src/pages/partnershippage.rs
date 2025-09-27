use yew::prelude::*;

#[function_component(PartnershipPage)]
pub fn partnership_page() -> Html {
    html! {
        <div class="partnership-container">
            <div class="page-header">
                <h1>{"Partnership"}</h1>
                <p>{"Collaborate with global and local organizations to build a stronger Stellar ecosystem"}</p>
            </div>

            <div class="partnership-content">
                <div class="partnership-section">
                    <h2>{"Our Partners"}</h2>
                    <div class="partners-grid">
                        <div class="partner-card">
                            <div class="partner-logo">{"WESTLAND"}</div>
                            <p>{"Leading blockchain infrastructure provider"}</p>
                        </div>
                        <div class="partner-card">
                            <div class="partner-logo">{"WESTLAND"}</div>
                            <p>{"Global fintech innovation platform"}</p>
                        </div>
                        <div class="partner-card">
                            <div class="partner-logo">{"WESTLAND"}</div>
                            <p>{"European blockchain development fund"}</p>
                        </div>
                        <div class="partner-card">
                            <div class="partner-logo">{"WESTLAND"}</div>
                            <p>{"Cross-border payment solutions"}</p>
                        </div>
                    </div>
                </div>

                <div class="partnership-section">
                    <h2>{"Become a Partner"}</h2>
                    <p class="partnership-description">
                        {"Join forces with Stellar Europe to advance blockchain adoption across the continent. "}
                        {"Our partnership program offers unique opportunities for collaboration, innovation, and growth."}
                    </p>
                    <button class="partner-cta">{"Apply for Partnership"}</button>
                </div>
            </div>

            <style>
                {r#"
                .partnership-container {
                    padding: 120px 20px;
                    background: #0F0F0F;
                    color: white;
                    min-height: 100vh;
                }

                .page-header {
                    text-align: center;
                    margin-bottom: 80px;
                }

                .page-header h1 {
                    font-size: 3rem;
                    color: #FFDA00;
                    margin-bottom: 20px;
                    text-transform: uppercase;
                }

                .page-header p {
                    font-size: 1.2rem;
                    color: #ccc;
                    max-width: 600px;
                    margin: 0 auto;
                }

                .partnership-content {
                    max-width: 1200px;
                    margin: 0 auto;
                }

                .partnership-section {
                    margin-bottom: 80px;
                }

                .partnership-section h2 {
                    font-size: 2.5rem;
                    color: #FFDA00;
                    text-align: center;
                    margin-bottom: 50px;
                    text-transform: uppercase;
                }

                .partners-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                    gap: 30px;
                    margin-bottom: 80px;
                }

                .partner-card {
                    background: rgba(255, 255, 255, 0.05);
                    border-radius: 15px;
                    padding: 40px 30px;
                    text-align: center;
                    border: 1px solid #333;
                    transition: transform 0.3s ease;
                }

                .partner-card:hover {
                    transform: translateY(-10px);
                    border-color: #FFDA00;
                }

                .partner-logo {
                    background: white;
                    color: #0F0F0F;
                    padding: 15px 30px;
                    border-radius: 8px;
                    font-weight: 700;
                    font-size: 1.2rem;
                    margin: 0 auto 20px;
                    display: inline-block;
                }

                .partner-card p {
                    color: #ccc;
                    line-height: 1.6;
                }

                .partnership-description {
                    font-size: 1.2rem;
                    color: #ccc;
                    text-align: center;
                    max-width: 700px;
                    margin: 0 auto 40px;
                    line-height: 1.8;
                }

                .partner-cta {
                    background: #FFDA00;
                    color: #0F0F0F;
                    border: none;
                    padding: 18px 36px;
                    border-radius: 25px;
                    font-weight: 700;
                    font-size: 1.1rem;
                    cursor: pointer;
                    transition: transform 0.3s ease;
                    display: block;
                    margin: 0 auto;
                }

                .partner-cta:hover {
                    transform: translateY(-3px);
                    box-shadow: 0 10px 25px rgba(255, 218, 0, 0.3);
                }
                "#}
            </style>
        </div>
    }
}