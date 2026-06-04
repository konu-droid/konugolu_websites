use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum Page {
    Home,
    Videos,
}

#[function_component(App)]
fn app() -> Html {
    let current_page = use_state(|| Page::Home);

    let set_page_home = {
        let current_page = current_page.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            current_page.set(Page::Home);
        })
    };

    let set_page_videos = {
        let current_page = current_page.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            current_page.set(Page::Videos);
        })
    };

    let skills = vec![
        "Deep Learning", "Reinforcement Learning", "PyTorch", "Git", "OpenCV", 
        "Python", "C++", "Linux", "ROS", "ROS2", "Docker", "NVIDIA Isaac Sim", "Gazebo"
    ];

    html! {
        <>
            <div class="bg-glow">
                <div class="glow-orb orb-1"></div>
                <div class="glow-orb orb-2"></div>
            </div>

            <nav>
                <div class="container nav-content">
                    <a onclick={set_page_home.clone()} class="nav-brand">{"MK."}</a>
                    <div class="nav-links">
                        <a onclick={set_page_home.clone()} class={if *current_page == Page::Home { "active" } else { "" }}>{"Home"}</a>
                        <a onclick={set_page_videos.clone()} class={if *current_page == Page::Videos { "active" } else { "" }}>{"YouTube Videos"}</a>
                    </div>
                </div>
            </nav>

            <main class="container">
                {
                    match *current_page {
                        Page::Home => html! {
                            <>
                                <section class="hero">
                                    <div class="hero-content">
                                        <div class="hero-text">
                                            <h1>{"Hi, I'm "}<span>{"Mohan"}</span><br/>{"Robotics Engineer"}</h1>
                                            <p>
                                                {"Seasoned robotics engineer with over 4 years of hands-on experience developing and deploying mobile robots across diverse industries. My expertise lies in rapid prototyping and efficient deployment of robotic solutions."}
                                            </p>
                                            <div class="hero-buttons">
                                                <a href="mailto:konudroid@gmail.com" class="btn btn-primary">{"Contact Me"}</a>
                                                <a href="https://www.linkedin.com/in/kvs-mohan-vamsi-1b2a52120/" class="btn btn-outline" target="_blank">{"LinkedIn"}</a>
                                                <a onclick={set_page_videos.clone()} class="btn btn-primary btn-pulse">{"🎬 Watch My Videos"}</a>
                                            </div>
                                        </div>
                                        <div class="hero-image">
                                            <img src="public/profile.jpg" alt="Mohan Konugolu Profile" />
                                        </div>
                                    </div>
                                </section>

                                <section class="section" id="skills">
                                    <h2 class="section-title">{"Core Skills"}</h2>
                                    <div class="skills-container">
                                        {
                                            for skills.iter().map(|skill| {
                                                html! { <span class="skill-tag">{skill}</span> }
                                            })
                                        }
                                    </div>
                                </section>

                                <section class="section" id="projects">
                                    <h2 class="section-title">{"Projects"}</h2>
                                    
                                    <h3 class="projects-header">{"Professional Projects"}</h3>
                                    <div class="timeline">
                                        <div class="timeline-item glass-card">
                                            <span class="timeline-date">{"Feb 2024 - Present"}</span>
                                            <h3 class="timeline-title">{"Custom Humanoid Simulation & RL Locomotion"}</h3>
                                            <div class="timeline-company">{"Upwork / Hyperspawn (Open Source)"}</div>
                                            <p>{"Imported an open source humanoid by hyperspawn with its complex 4-bar mechanisms and stuart platform and then trained the robot for locomotion in isaac sim using isaaclab."}</p>
                                            <ul>
                                                <li>{"Solved complex kinematic linkages."}</li>
                                                <li>{"Trained humanoid to walk using RL."}</li>
                                                <li>{"Mentioned as contributor in hyperspawn open source project."}</li>
                                            </ul>
                                        </div>

                                        <div class="timeline-item glass-card">
                                            <span class="timeline-date">{"Sep 2022 - Feb 2024"}</span>
                                            <h3 class="timeline-title">{"Depth Estimation Using Mono-Camera"}</h3>
                                            <div class="timeline-company">{"Bosch (BGSW)"}</div>
                                            <p>{"Generated absolute object level depth from a monocular camera by training a deep learning model. Achieved a MAE of 0.18m on custom dataset."}</p>
                                            <ul>
                                                <li>{"Invited for poster presentation in Bosch Germany AAI conference."}</li>
                                            </ul>
                                        </div>

                                        <div class="timeline-item glass-card">
                                            <span class="timeline-date">{"Aug 2021 - Aug 2022"}</span>
                                            <h3 class="timeline-title">{"Autonomous Mobile Robot Software Stack"}</h3>
                                            <div class="timeline-company">{"ZeroPoint Robotics"}</div>
                                            <p>{"Worked on the complete robot stack from safety code to motion planner code as a core software engineer."}</p>
                                            <ul>
                                                <li>{"Developed double look ahead path tracking (pure pursuit)."}</li>
                                                <li>{"Implemented continuous curve algorithms (clothoid) and TEB."}</li>
                                                <li>{"Coded laser-based safety and trolley leg detection using 2D lidar."}</li>
                                            </ul>
                                        </div>
                                    </div>

                                    <h3 class="projects-header">{"Personal & Academic Projects"}</h3>
                                    <div class="grid-2">
                                        <div class="project-card glass-card">
                                            <span class="project-date">{"Recent"}</span>
                                            <h3>{"Brick Laying Using Reinforcement Learning"}</h3>
                                            <div class="project-company">{"Personal Project"}</div>
                                            <p>{"Trained a simulation of an industrial arm with 5 fingered humanoid hand to pick and place bricks using reinforcement learning."}</p>
                                            <ul>
                                                <li>{"Created virtual environment in Isaac sim."}</li>
                                                <li>{"Used SKRL library and PPO algorithm."}</li>
                                            </ul>
                                        </div>
                                    </div>
                                </section>

                                <section class="section" id="education">
                                    <h2 class="section-title">{"Education"}</h2>
                                    <div class="grid-2">
                                        <div class="glass-card">
                                            <span class="timeline-date">{"Sep 2024 - Sep 2025"}</span>
                                            <h3>{"MSc. Robotics and Artificial Intelligence"}</h3>
                                            <p>{"Cardiff Metropolitan University"}<br/>{"Cardiff, United Kingdom"}</p>
                                        </div>
                                        <div class="glass-card">
                                            <span class="timeline-date">{"Jul 2016 - Jul 2020"}</span>
                                            <h3>{"BEng. Mechatronics"}</h3>
                                            <p>{"Chandigarh University"}<br/>{"Punjab, India"}</p>
                                        </div>
                                    </div>
                                </section>
                            </>
                        },
                        Page::Videos => html! {
                            <section class="section">
                                <h2 class="section-title">{"My YouTube Videos"}</h2>
                                <p style="text-align: center; color: var(--text-secondary); margin-bottom: 40px;">
                                    {"Check out my latest robotics and reinforcement learning videos on my channel."}
                                </p>
                                <div class="video-grid">
                                    <div class="video-card">
                                        <div class="video-container">
                                            <iframe src="https://www.youtube.com/embed/KdVAOl16WMg?enablejsapi=1" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen" allowfullscreen=true></iframe>
                                        </div>
                                        <div class="video-info">
                                            <h3>{"Robotics & Reinforcement Learning Project 1"}</h3>
                                            <p>{"A deep dive into one of my recent robotics simulations and RL training setups."}</p>
                                        </div>
                                    </div>
                                    
                                    <div class="video-card">
                                        <div class="video-container">
                                            <iframe src="https://www.youtube.com/embed/U9-3ey494_Y?enablejsapi=1" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen" allowfullscreen=true></iframe>
                                        </div>
                                        <div class="video-info">
                                            <h3>{"Robotics & Reinforcement Learning Project 2"}</h3>
                                            <p>{"Demonstrating custom humanoid locomotion trained using Isaac Sim and IsaacLab."}</p>
                                        </div>
                                    </div>

                                    <div class="video-card">
                                        <div class="video-container">
                                            <iframe src="https://www.youtube.com/embed/iXFA17m4LQI?enablejsapi=1" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen" allowfullscreen=true></iframe>
                                        </div>
                                        <div class="video-info">
                                            <h3>{"Robotics & Reinforcement Learning Project 3"}</h3>
                                            <p>{"Showcasing advanced path planning and reinforcement learning algorithms in action."}</p>
                                        </div>
                                    </div>
                                </div>

                                <h2 class="section-title" style="margin-top: 80px;">{"YouTube Shorts"}</h2>
                                <div class="shorts-grid">
                                    <div class="video-card">
                                        <div class="short-container">
                                            <iframe src="https://www.youtube.com/embed/BrIw-jcCLx4?enablejsapi=1" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen" allowfullscreen=true></iframe>
                                        </div>
                                        <div class="video-info">
                                            <h3>{"Isaac Sim Overview"}</h3>
                                            <p>{"A quick look into setting up robotic simulations in NVIDIA Isaac Sim."}</p>
                                        </div>
                                    </div>
                                    <div class="video-card">
                                        <div class="short-container">
                                            <iframe src="https://www.youtube.com/embed/ETaIXZGZtn0?enablejsapi=1" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen" allowfullscreen=true></iframe>
                                        </div>
                                        <div class="video-info">
                                            <h3>{"RL Training Progress"}</h3>
                                            <p>{"Tracking the training progress of the humanoid robot using PPO."}</p>
                                        </div>
                                    </div>
                                    <div class="video-card">
                                        <div class="short-container">
                                            <iframe src="https://www.youtube.com/embed/PRNMIgR9ico?enablejsapi=1" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen" allowfullscreen=true></iframe>
                                        </div>
                                        <div class="video-info">
                                            <h3>{"Kinematic Linkages"}</h3>
                                            <p>{"Demonstrating the complex 4-bar mechanism solver in action."}</p>
                                        </div>
                                    </div>
                                    <div class="video-card">
                                        <div class="short-container">
                                            <iframe src="https://www.youtube.com/embed/gcxUFqX_c8s?enablejsapi=1" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen" allowfullscreen=true></iframe>
                                        </div>
                                        <div class="video-info">
                                            <h3>{"Robot Deployment"}</h3>
                                            <p>{"Short clip showing the mobile robot software stack safely navigating."}</p>
                                        </div>
                                    </div>
                                </div>

                                <div style="text-align: center; margin-top: 60px;">
                                    <a href="https://youtube.com/@konudroid?feature=shared" target="_blank" class="btn btn-primary">{"Visit Full Channel"}</a>
                                </div>
                            </section>
                        }
                    }
                }
            </main>

            <footer>
                <div class="container">
                    <div class="social-links">
                        <a href="mailto:konudroid@gmail.com">{"Email"}</a>
                        <a href="https://www.linkedin.com/in/kvs-mohan-vamsi-1b2a52120/" target="_blank">{"LinkedIn"}</a>
                        <a href="https://youtube.com/@konudroid?feature=shared" target="_blank">{"YouTube"}</a>
                        <a href="https://www.upwork.com/freelancers/~0106f44a845518bf78?mp_source=share" target="_blank">{"Upwork"}</a>
                        <a href="https://github.com/Hyperspawn/dropbear_isaac" target="_blank">{"GitHub"}</a>
                    </div>
                    <p style="color: var(--text-secondary); font-size: 14px;">
                        {"© 2026 Mohan Konugolu. Built with Rust and Yew."}
                    </p>
                </div>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
