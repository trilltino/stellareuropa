use yew::prelude::*;

#[function_component(ChaptersPage)]
pub fn chapters_page() -> Html {
    html! {
        <div class="chapters-container">
            <div class="page-header">
                <h1>{"Stellar Europe Chapters"}</h1>
                <p>{"Connect with local Stellar communities across Europe"}</p>
            </div>

            <div class="chapters-grid">
                <div class="chapter-card">
                    <img src="/assets/uk-chapter.jpg" alt="UK Chapter" loading="lazy" />
                    <h3>{"UK Chapter"}</h3>
                    <p>{"Join the thriving Stellar community in the United Kingdom"}</p>
                    <button class="join-btn">{"Join Chapter"}</button>
                </div>

                <div class="chapter-card">
                    <img src="/assets/ireland-chapter.jpg" alt="Ireland Chapter" loading="lazy" />
                    <h3>{"Ireland Chapter"}</h3>
                    <p>{"Connect with builders and innovators in Ireland"}</p>
                    <button class="join-btn">{"Join Chapter"}</button>
                </div>

                <div class="chapter-card">
                    <img src="/assets/dach-chapter.jpg" alt="DACH Chapter" loading="lazy" />
                    <h3>{"DACH Chapter"}</h3>
                    <p>{"Germany, Austria, and Switzerland Stellar community"}</p>
                    <button class="join-btn">{"Join Chapter"}</button>
                </div>

                <div class="chapter-card">
                    <img src="/assets/france-chapter.jpg" alt="France Chapter" loading="lazy" />
                    <h3>{"France Chapter"}</h3>
                    <p>{"Join the French Stellar ecosystem"}</p>
                    <button class="join-btn">{"Join Chapter"}</button>
                </div>
            </div>

            <style>
                {r#"
                .chapters-container {
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
                }

                .page-header p {
                    font-size: 1.2rem;
                    color: #ccc;
                }

                .chapters-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 30px;
                    max-width: 1200px;
                    margin: 0 auto;
                }

                .chapter-card {
                    background: rgba(255, 255, 255, 0.05);
                    border-radius: 15px;
                    padding: 30px;
                    text-align: center;
                    border: 1px solid #333;
                    transition: transform 0.3s ease;
                }

                .chapter-card:hover {
                    transform: translateY(-10px);
                    border-color: #FFDA00;
                }

                .chapter-card img {
                    width: 100%;
                    height: 200px;
                    object-fit: cover;
                    border-radius: 10px;
                    margin-bottom: 20px;
                }

                .chapter-card h3 {
                    color: #FFDA00;
                    font-size: 1.5rem;
                    margin-bottom: 15px;
                }

                .chapter-card p {
                    color: #ccc;
                    margin-bottom: 25px;
                }

                .join-btn {
                    background: #FFDA00;
                    color: #0F0F0F;
                    border: none;
                    padding: 12px 24px;
                    border-radius: 25px;
                    font-weight: 600;
                    cursor: pointer;
                    transition: transform 0.3s ease;
                }

                .join-btn:hover {
                    transform: translateY(-2px);
                }
                "#}
            </style>
        </div>
    }
}