use yew::prelude::*;
use gloo::timers::callback::Interval;

const NATIONS: [&str; 8] = [
    "Stellar Europe",
    "Ireland",
    "Germany",
    "Iberia",
    "France",
    "United Kingdom",
    "Cyprus",
    "Austria",
];

const TAGLINE: &str = "Think global build on Stellar";

#[function_component(HomePage)]
pub fn homepage() -> Html {
    let current_index = use_state(|| 0);

    use_effect_with(current_index.clone(), {
        let current_index = current_index.clone();
        move |_| {
            let interval = Interval::new(3000, move || {
                // Total items: 8 nations + 1 tagline = 9 items (0-8)
                let next = (*current_index + 1) % (NATIONS.len() + 1);
                current_index.set(next);
            });

            // Return cleanup function that drops interval when component unmounts
            move || drop(interval)
        }
    });

    let is_tagline = *current_index == NATIONS.len();
    let display_text = if is_tagline {
        TAGLINE
    } else {
        NATIONS[*current_index]
    };

    html! {
        <div class="homepage">
            <div class="hero-section">
                <div class="hero-content">
                    <h1 class="hero-title">
                        <span class="rotating-text" key={*current_index}>{display_text}</span>
                    </h1>
                    if !is_tagline {
                        <p class="hero-subtitle">{"Think global build on Stellar."}</p>
                    }
                </div>

                <div class="event-slideshow">
                <p class="slideshow-title">{"A continent of experts, join the europe chapter and kickstart your journey"}</p>
                <div class="slideshow-track">
                    // First 3 images load immediately for smooth initial render
                    <img src="/eventphotos/a.webp" alt="Event" class="slideshow-image" />
                    <img src="/eventphotos/d.webp" alt="Event" class="slideshow-image" />
                    <img src="/eventphotos/e.webp" alt="Event" class="slideshow-image" />
                    // Rest load lazily to improve initial page load
                    <img src="/eventphotos/f.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/g.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/h.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/i.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/o.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/q.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/r.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/s.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/t.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/u.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/w.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/y.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    // Duplicate for seamless loop
                    <img src="/eventphotos/a.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/d.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/e.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/f.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/g.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/h.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/i.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/o.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/q.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/r.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/s.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/t.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/u.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/w.webp" alt="Event" class="slideshow-image" loading="lazy" />
                    <img src="/eventphotos/y.webp" alt="Event" class="slideshow-image" loading="lazy" />
                </div>
                </div>
            </div>

            <div class="mission-section">
                <h2 class="mission-title">{"Our Mission"}</h2>
                <p class="mission-description">
                    {"We are people focused, and believe in the power of individuals to determine their financial access. We leverage local leaders, creators and speakers to further the goal of equitable finance."}
                </p>

                <div class="mission-photos">
                    <img src="/missonphotos/1.webp" alt="Our Mission" class="mission-photo" loading="lazy" />
                    <img src="/missonphotos/2.webp" alt="Our Mission" class="mission-photo" loading="lazy" />
                </div>
            </div>
        </div>
    }
}