use yew::prelude::*;

/// The two top-level views rendered by this single-page application.
#[derive(Clone, PartialEq)]
enum Page {
    Home,
    Videos,
}

/// A professional role displayed as an expandable card in the experience timeline.
struct Experience {
    /// Stable key linking this role to its related projects (`Project::employer_key`).
    key: &'static str,
    /// Human-readable employment period (e.g. "Feb 2024 – Present").
    period: &'static str,
    /// Job title for the role.
    title: &'static str,
    /// Company and location line.
    company: &'static str,
    /// One-sentence summary of the role.
    summary: &'static str,
    /// Bullet-point highlights/achievements for the role.
    highlights: &'static [&'static str],
}

/// A featured project displayed as a glass card in the projects grid and,
/// in compact form, inside its employer's expandable experience card.
struct Project {
    /// Key of the employer this project was delivered under (`Experience::key`).
    employer_key: &'static str,
    /// Short context label (e.g. "Freelance · Hyperspawn").
    context: &'static str,
    /// Project title.
    title: &'static str,
    /// Technology tags shown as small pills on the card.
    tech: &'static [&'static str],
    /// One-sentence description of the project.
    summary: &'static str,
    /// Bullet-point highlights for the project.
    highlights: &'static [&'static str],
}

/// A YouTube embed (full video or short) shown on the Videos page.
struct Video {
    /// YouTube embed id used to build the iframe `src`.
    embed_id: &'static str,
    /// Card title.
    title: &'static str,
    /// Short description shown under the embed.
    description: &'static str,
}

/// Core skills rendered as pill tags on the home page.
const SKILLS: [&str; 20] = [
    "Python", "C++", "ROS", "ROS2", "Nav2", "NVIDIA Isaac Sim", "IsaacLab",
    "Reinforcement Learning", "PPO", "SKRL", "Sim-to-Real", "PyTorch",
    "Deep Learning", "OpenCV", "6-DoF Pose Estimation", "SLAM",
    "Motion Planning", "Docker", "Git", "Linux",
];

/// Returns the professional experience entries, newest first.
fn experiences() -> Vec<Experience> {
    vec![
        Experience {
            key: "upwork",
            period: "Feb 2024 – Present",
            title: "Robotics Consultant (Freelance)",
            company: "Upwork · Remote",
            summary: "Top Rated robotics consultant (100% Job Success · 20 jobs · 597+ hours) delivering NVIDIA Isaac Sim simulation and reinforcement-learning solutions to robotics companies worldwide — every project containerized with Docker for reproducible deployment.",
            highlights: &[
                "Built large-scale Isaac Sim factory digital twins with coordinated AMRs, manipulators and simulated LiDAR/camera/IMU streams, wired to ROS2 and IsaacLab.",
                "Implemented a FoundationPose 6-DoF object-pose pipeline using NVIDIA Isaac ROS.",
                "Trained humanoid locomotion and whole-body gesture control (Unitree G1) with the HOVER framework and SMPL-X motion retargeting.",
            ],
        },
        Experience {
            key: "bosch",
            period: "Sep 2022 – Feb 2024",
            title: "Software Developer",
            company: "Bosch Global Software Technologies (BGSW) · India",
            summary: "Developed machine learning for autonomous driving, focused on computer-vision depth perception and reinforcement-learning vehicle control.",
            highlights: &[
                "Trained a monocular depth model (YOLO + MiDaS) reaching 0.18 m MAE on a custom dataset.",
                "Selected for a poster presentation at Bosch Germany's internal AAI research conference.",
                "Built LiDAR cover-glass blockage detection (MiniRocket & MobileNetV3) above 90% accuracy.",
            ],
        },
        Experience {
            key: "zeropoint",
            period: "Aug 2021 – Aug 2022",
            title: "Robotics Engineer",
            company: "ZeroPoint Robotics · India",
            summary: "Core software engineer for an autonomous material-handling mobile robot, owning the full stack from safety to motion planning.",
            highlights: &[
                "Developed a double-look-ahead pure-pursuit path tracker and clothoid continuous-curvature path generation.",
                "Deployed an industry-standard Timed Elastic Band (TEB) local planner for efficient navigation.",
                "Coded 3-zone 2D-LiDAR safety monitoring and trolley-leg detection for autonomous docking.",
            ],
        },
        Experience {
            key: "thrsl",
            period: "Dec 2020 – Jul 2021",
            title: "Deployment Engineer",
            company: "THRSL · India",
            summary: "Developed pallet-detection perception and ran on-site deployment of autonomous pallet jacks in a working factory.",
            highlights: &[
                "Set up 2D-LiDAR mapping, localization, route verification and motion-controller tuning.",
                "Tuned route-following and docking behaviour and coordinated on-site acceptance testing.",
                "Validated safety sensors and ran extended long-term operation across predetermined routes.",
            ],
        },
    ]
}

/// Returns every featured project from the CV, newest employer first.
/// Each project links back to its employer via `employer_key` so the
/// experience timeline can surface its related projects when expanded.
fn projects() -> Vec<Project> {
    vec![
        Project {
            employer_key: "upwork",
            context: "Freelance · Hyperspawn (Open Source)",
            title: "Custom Humanoid Simulation & RL Locomotion",
            tech: &["Isaac Sim", "Isaac Lab", "PyTorch", "PPO"],
            summary: "Imported an open-source humanoid with complex 4-bar linkages and a Stewart platform into Isaac Sim and trained it to walk with reinforcement learning.",
            highlights: &[
                "Solved the closed-chain kinematics so the simulated dynamics matched the real robot.",
                "Trained a bipedal walking policy with RL and an SMPL-X motion-retargeting (PHC) pipeline for combined imitation + RL.",
                "Credited as a contributor to the Hyperspawn open-source project.",
            ],
        },
        Project {
            employer_key: "upwork",
            context: "Freelance · Unitree G1",
            title: "Humanoid Whole-Body Gesture Control",
            tech: &["HOVER", "SMPL-X", "Isaac Lab"],
            summary: "Trained the Unitree G1 humanoid to perform naturalistic, human-like conversational gestures using the HOVER whole-body-control framework.",
            highlights: &[
                "Adapted the motion-retargeting pipeline to ingest SMPL-X data and retarget human motion onto the G1.",
                "Extended the task to five-fingered hands for lifelike hand movement during conversation.",
            ],
        },
        Project {
            employer_key: "upwork",
            context: "Freelance · Reinforcement Learning",
            title: "Dexterous Brick-Laying Manipulation",
            tech: &["Isaac Sim", "Isaac Lab", "SKRL", "PPO"],
            summary: "Trained a five-fingered industrial arm in Isaac Sim to approach, grasp, lift and place bricks using reinforcement learning.",
            highlights: &[
                "Built a custom environment with fingertip force sensors and IMUs for closed-loop, contact-aware grasping.",
                "Trained with PPO (SKRL) for a smooth approach-grasp-lift-place sequence.",
                "Tuned the reward function for reliable placement accuracy.",
            ],
        },
        Project {
            employer_key: "upwork",
            context: "Freelance · Autonomous Navigation",
            title: "RL Motion Control for Autonomous Navigation",
            tech: &["Isaac Sim", "ROS 2", "Nav2"],
            summary: "Integrated a reinforcement-learning motion-control policy with the Nav2 stack for smooth low-speed navigation and pedestrian avoidance.",
            highlights: &[
                "Built a custom-vehicle Isaac Sim scene (LiDAR, camera, IMU, motor control) bridged to ROS 2.",
                "Added custom low-speed smoothing for stable steering.",
                "Trained and validated point-to-point policies that avoid slow-moving pedestrians.",
            ],
        },
        Project {
            employer_key: "upwork",
            context: "Freelance · Digital Twins",
            title: "Robot Factory Digital Twin",
            tech: &["Isaac Sim", "ROS 2", "Isaac Lab"],
            summary: "Built physics-accurate Isaac Sim digital twins of robot factories with multiple coordinated robots and simulated sensor streams.",
            highlights: &[
                "Simulated LiDAR, camera and IMU streams across coordinated AMRs and manipulators.",
                "Wired the twin to ROS 2 and Isaac Lab so client teams could validate control logic and train RL policies before hardware rollout.",
            ],
        },
        Project {
            employer_key: "upwork",
            context: "Freelance · Perception",
            title: "Foundation-Model 6-DoF Pose Pipeline",
            tech: &["NVIDIA Isaac ROS", "FoundationPose"],
            summary: "Implemented a FoundationPose 6-DoF object pose-estimation and tracking pipeline using NVIDIA Isaac ROS.",
            highlights: &[
                "Integrated the pose output into Isaac Sim and a ROS 2 perception graph for a client perception system.",
            ],
        },
        Project {
            employer_key: "bosch",
            context: "Bosch · Autonomous Driving",
            title: "Monocular Depth Estimation",
            tech: &["PyTorch", "YOLO", "MiDaS"],
            summary: "Trained a deep network combining YOLO boxes with MiDaS relative depth for absolute object-level depth estimation.",
            highlights: &[
                "Achieved 0.18 m MAE in-house and 1.1 m on the nuScenes dataset.",
                "Selected for a poster at Bosch Germany's internal AAI research conference.",
            ],
        },
        Project {
            employer_key: "bosch",
            context: "Bosch · LiDAR Machine Learning",
            title: "LiDAR Cover-Glass Blockage Detection",
            tech: &["PyTorch", "MiniRocket", "MobileNetV3"],
            summary: "Detected cover-glass occlusion on a solid-state LiDAR using two complementary deep-learning approaches.",
            highlights: &[
                "MiniRocket on photon time-series and MobileNetV3 on 3-channel point-cloud images, both above 90% accuracy.",
                "Compared both approaches and selected the stronger model for deployment.",
            ],
        },
        Project {
            employer_key: "zeropoint",
            context: "ZeroPoint · Mobile Robotics",
            title: "Autonomous Mobile Robot Software Stack",
            tech: &["C++", "ROS", "2D LiDAR", "TEB"],
            summary: "Built a full AMR autonomy stack from path tracking and planning through to LiDAR safety and docking.",
            highlights: &[
                "Developed double-look-ahead pure-pursuit tracking and clothoid continuous-curvature path generation.",
                "Deployed an industry-standard Timed Elastic Band (TEB) local planner.",
                "Coded 3-zone 2D-LiDAR safety monitoring and trolley-leg detection for autonomous docking.",
            ],
        },
        Project {
            employer_key: "thrsl",
            context: "THRSL · On-Site Deployment",
            title: "Autonomous Pallet-Jack Commissioning",
            tech: &["2D LiDAR", "SLAM", "Perception"],
            summary: "Commissioned two autonomous battery-operated pallet jacks in a live factory, from mapping through to acceptance testing.",
            highlights: &[
                "Set up 2D-LiDAR mapping, localization, route verification and motion-controller tuning.",
                "Built pallet-detection perception and tuned route-following and docking behaviour.",
                "Validated safety sensors and ran extended long-term operation across predetermined routes.",
            ],
        },
    ]
}

/// Returns the full-length videos shown on the Videos page.
fn videos() -> Vec<Video> {
    vec![
        Video {
            embed_id: "iXFA17m4LQI",
            title: "I Started Freelancing as a Roboticist!",
            description: "This is my first video in the series of videos about freelancing a robotics engineer, i share what i did and how much i earned",
        },
        Video {
            embed_id: "U9-3ey494_Y",
            title: "Simulators don't have air, Then how does a drone fly?",
            description: "This video details how does a drone fly in a simulation even though no air is simulated. ",
        },
    ]
}

/// Returns the short-form videos shown on the Videos page.
fn shorts() -> Vec<Video> {
    vec![
        Video {
            embed_id: "gcxUFqX_c8s",
            title: "Fix Robot Crazy Movement in ROS2 | KDL vs Trac_IK Explained",
            description: "My robot kept spinning out during pick and place",
        },
        Video {
            embed_id: "z98FERxxiV0",
            title: "AI learned what I told it to do, not what I wanted it to do.",
            description: "It turns out my reward function was a bit too focused ",
        },
        Video {
            embed_id: "xmn7SLYRrLw",
            title: "Making snap-fit parts in Isaac Sim!",
            description: "A recent freelancing job 💼 pushed me to explore snap-fit parts in Isaac Sim.",
        },
        Video {
            embed_id: "ETaIXZGZtn0",
            title: "Is your mobile robot dancing (wobbling) in Isaac Sim?!?!",
            description: "While working with wheeled robots in Isaac Sim, have you ever experienced:",
        },
    ]
}

/// Renders a list of string highlights as `<li>` bullet items.
fn highlight_items(highlights: &[&'static str]) -> Html {
    html! {
        { for highlights.iter().map(|item| html! { <li>{ *item }</li> }) }
    }
}

/// Renders a project's technology tags as a row of small pills.
fn tech_tags(tech: &[&'static str]) -> Html {
    html! {
        <div class="project-tech">
            { for tech.iter().map(|tag| html! { <span class="tech-tag">{ *tag }</span> }) }
        </div>
    }
}

/// Renders a compact project entry shown inside an expanded experience card.
fn related_project_item(project: &Project) -> Html {
    html! {
        <div class="related-project">
            <h4>{ project.title }</h4>
            { tech_tags(project.tech) }
            <p>{ project.summary }</p>
        </div>
    }
}

/// Renders a single experience entry as an expandable glass timeline card.
///
/// The card header (period, title, company, summary) lives inside a
/// `<summary>` so clicking anywhere on it toggles the body, which reveals
/// the role highlights plus the projects delivered under that employer.
fn experience_card(exp: &Experience) -> Html {
    let related: Vec<Project> = projects()
        .into_iter()
        .filter(|project| project.employer_key == exp.key)
        .collect();
    html! {
        <details class="timeline-item glass-card experience-card">
            <summary class="experience-summary">
                <div class="experience-heading">
                    <span class="timeline-date">{ exp.period }</span>
                    <h3 class="timeline-title">{ exp.title }</h3>
                    <div class="timeline-company">{ exp.company }</div>
                    <p class="experience-blurb">{ exp.summary }</p>
                    <span class="expand-hint">
                        <span class="expand-chevron" aria-hidden="true">{"▾"}</span>
                        { format!(" Highlights & {} related project{}",
                            related.len(), if related.len() == 1 { "" } else { "s" }) }
                    </span>
                </div>
            </summary>
            <div class="experience-body">
                <ul>{ highlight_items(exp.highlights) }</ul>
                <h4 class="related-heading">{"Related Projects"}</h4>
                <div class="related-projects">
                    { for related.iter().map(related_project_item) }
                </div>
            </div>
        </details>
    }
}

/// Renders a single featured project as a glass card with tech tags.
fn project_card(project: &Project) -> Html {
    html! {
        <div class="project-card glass-card">
            <div class="project-company">{ project.context }</div>
            <h3>{ project.title }</h3>
            { tech_tags(project.tech) }
            <p>{ project.summary }</p>
            <ul>{ highlight_items(project.highlights) }</ul>
        </div>
    }
}

/// Renders a single video card.
///
/// `container_class` selects the aspect-ratio wrapper: `video-container`
/// for 16:9 videos or `short-container` for 9:16 shorts.
fn video_card(video: &Video, container_class: &'static str) -> Html {
    let src = format!(
        "https://www.youtube.com/embed/{}?enablejsapi=1",
        video.embed_id
    );
    html! {
        <div class="video-card">
            <div class={classes!(container_class)}>
                <iframe src={src}
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen"
                    allowfullscreen=true></iframe>
            </div>
            <div class="video-info">
                <h3>{ video.title }</h3>
                <p>{ video.description }</p>
            </div>
        </div>
    }
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
                                                {"Robotics engineer with 5+ years across robotics consulting, autonomous driving and mobile-robot startups. I build digital twins and reinforcement-learning pipelines in NVIDIA Isaac Sim — a Top Rated freelance consultant deploying robotic solutions for companies worldwide."}
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
                                            for SKILLS.iter().map(|skill| {
                                                html! { <span class="skill-tag">{skill}</span> }
                                            })
                                        }
                                    </div>
                                </section>

                                <section class="section" id="experience">
                                    <h2 class="section-title">{"Experience"}</h2>
                                    <p class="section-hint">{"Click a role to see its highlights and related projects."}</p>
                                    <div class="timeline">
                                        { for experiences().iter().map(experience_card) }
                                    </div>
                                </section>

                                <section class="section" id="projects">
                                    <h2 class="section-title">{"Featured Projects"}</h2>
                                    <div class="projects-grid">
                                        { for projects().iter().map(project_card) }
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
                                            <span class="timeline-date">{"2016 - 2020"}</span>
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
                                    { for videos().iter().map(|v| video_card(v, "video-container")) }
                                </div>

                                <h2 class="section-title" style="margin-top: 80px;">{"YouTube Shorts"}</h2>
                                <div class="shorts-grid">
                                    { for shorts().iter().map(|v| video_card(v, "short-container")) }
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
                        <a href="https://github.com/konu-droid" target="_blank">{"GitHub"}</a>
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
