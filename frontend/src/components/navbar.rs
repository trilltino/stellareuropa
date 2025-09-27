use yew::prelude::*;
use yew_router::prelude::*;
use crate::routing::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <div class="nav-container">
                <div class="nav-left">
                    <Link<Route> to={Route::Home} classes="nav-brand">
                        <img src="/brandlogo.png" alt="Stellar Europe Logo" class="brand-logo" />
                    </Link<Route>>
                </div>

                <div class="nav-center">
                </div>

                <div class="nav-right">
                    <div class="nav-links">
                        <Link<Route> to={Route::Home} classes="nav-link">
                            {"Home"}
                        </Link<Route>>
                        <Link<Route> to={Route::Events} classes="nav-link">
                            {"Events"}
                        </Link<Route>>
                    </div>
                </div>
            </div>

            <style>
                {r#"
                .navbar {
                    background: transparent;
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    height: 70px;
                    z-index: 1000;
                    backdrop-filter: blur(10px);
                }

                .nav-container {
                    display: flex;
                    width: 1556.253px;
                    justify-content: space-between;
                    align-items: center;
                    margin: 0 auto;
                    height: 100%;
                    padding: 0 20px;
                }

                .nav-left {
                    flex: 1;
                }

                .nav-brand {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    font-size: 1.8rem;
                    font-weight: bold;
                    color: #00d4ff;
                    text-decoration: none;
                    background: linear-gradient(45deg, #00d4ff, #0099cc);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                }

                .brand-logo {
                    width: 83px;
                    height: 83px;
                    flex-shrink: 0;
                    aspect-ratio: 1/1;
                }

                .nav-brand:hover {
                    transform: scale(1.05);
                    transition: transform 0.3s ease;
                }

                .nav-center {
                    flex: 1;
                    display: flex;
                    justify-content: center;
                }

                .nav-links {
                    display: flex;
                    gap: 30px;
                    align-items: center;
                }

                .nav-link {
                    color: white;
                    text-decoration: none;
                    font-family: 'Inter', 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    font-weight: 500;
                    font-size: 1rem;
                    padding: 8px 16px;
                    border-radius: 6px;
                    transition: all 0.3s ease;
                    position: relative;
                }

                .nav-link:hover {
                    color: #00d4ff;
                    background-color: rgba(0, 212, 255, 0.1);
                    transform: translateY(-2px);
                }

                .nav-link:hover::after {
                    content: '';
                    position: absolute;
                    bottom: -2px;
                    left: 50%;
                    transform: translateX(-50%);
                    width: 80%;
                    height: 2px;
                    background: linear-gradient(45deg, #00d4ff, #0099cc);
                    border-radius: 1px;
                }

                .nav-icon {
                    height: 30px;
                    width: auto;
                    transition: all 0.3s ease;
                }

                .nav-icon:hover {
                    transform: scale(1.1);
                    filter: brightness(1.2);
                }

                .nav-join-us {
                    display: flex;
                    align-items: center;
                    text-decoration: none;
                    transition: all 0.3s ease;
                }

                .nav-join-us:hover {
                    transform: translateY(-2px);
                }

                .nav-join-icon {
                    height: 35px;
                    width: auto;
                    transition: all 0.3s ease;
                }

                .nav-join-icon:hover {
                    transform: scale(1.05);
                    filter: brightness(1.1);
                }

                .nav-right {
                    flex: 1;
                    display: flex;
                    justify-content: flex-end;
                }

                .nav-user-type {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    background: rgba(0, 212, 255, 0.05);
                    padding: 8px 16px;
                    border-radius: 8px;
                    border: 1px solid #333;
                }

                .user-type-label {
                    color: #ccc;
                    font-size: 0.9rem;
                    font-weight: 500;
                }

                .ambassador-link {
                    color: #00d4ff !important;
                    font-weight: 600;
                    padding: 6px 12px;
                    border: 1px solid #00d4ff;
                    border-radius: 6px;
                    font-size: 0.9rem;
                }

                .ambassador-link:hover {
                    background-color: #00d4ff;
                    color: black !important;
                    transform: translateY(-1px);
                    box-shadow: 0 4px 12px rgba(0, 212, 255, 0.3);
                }

                .chapter-lead-link {
                    color: #ff6b35 !important;
                    font-weight: 600;
                    padding: 6px 12px;
                    border: 1px solid #ff6b35;
                    border-radius: 6px;
                    font-size: 0.9rem;
                }

                .chapter-lead-link:hover {
                    background-color: #ff6b35;
                    color: black !important;
                    transform: translateY(-1px);
                    box-shadow: 0 4px 12px rgba(255, 107, 53, 0.3);
                }

                .separator {
                    color: #666;
                    font-size: 0.8rem;
                }

                /* Mobile Responsive */
                @media (max-width: 768px) {
                    .nav-container {
                        padding: 0 15px;
                        flex-direction: column;
                        height: auto;
                        min-height: 70px;
                        justify-content: center;
                        gap: 10px;
                    }

                    .nav-left, .nav-center, .nav-right {
                        flex: none;
                    }

                    .nav-links {
                        gap: 15px;
                    }

                    .nav-link {
                        font-size: 0.9rem;
                        padding: 6px 12px;
                    }

                    .nav-user-type {
                        gap: 8px;
                        padding: 6px 12px;
                    }

                    .user-type-label {
                        font-size: 0.8rem;
                    }

                    .ambassador-link, .chapter-lead-link {
                        font-size: 0.8rem;
                        padding: 4px 8px;
                    }
                }

                @media (max-width: 480px) {
                    .nav-brand {
                        font-size: 1.4rem;
                    }

                    .nav-links {
                        gap: 10px;
                    }

                    .nav-link {
                        font-size: 0.8rem;
                        padding: 4px 8px;
                    }
                }
                "#}
            </style>
        </nav>
    }
}