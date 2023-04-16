use leptos::*;

use crate::components::PageWrapper::*;

struct Project {
    title: String,
    description: String,
    link: String,
}

impl Project {
    fn new(title: String, description: String, link: String) -> Self {
        Self {
            title,
            description,
            link,
        }
    }
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let projects: Vec<Project> = vec![
        Project::new(
            String::from("Oxycards - Terminal based quiz cards"),
            String::from("Oxycards is a quiz card application built within the terminal."),
            String::from("https://github.com/BrookJeynes/oxycards"),
        ),
        Project::new(
            String::from("github.io/slime-rancher-2-interactive-map"),
            String::from("An interactive game map for the game Slime Rancher 2."),
            String::from("https://brookjeynes.github.io/slime-rancher-2-interactive-map/"),
        ),
        Project::new(
            String::from("github/brookjeynes"),
            String::from("A home for all my projects."),
            String::from("https://github.com/BrookJeynes"),
        ),
    ];

    view! { cx,
        <PageWrapper title={String::from("Brook-Jeynes")}>
            <div class="my-16">
                <h2 class="text-2xl font-medium my-4">"About me"</h2>
                <p>"Hi, I'm Brook, an aussie software developer predominantly writing programs in Rust, Typescript, and .NET"</p>
                <ul>
                </ul>
            </div>

            <div class="my-16">
                <h2 class="text-2xl font-medium my-4">"Posts"</h2>
            </div>

            <div class="my-16">
                <h2 class="text-2xl font-medium my-4">"Projects"</h2>
                <ul>
                {
                    projects.into_iter()
                        .map(|project| {
                            view! {
                                cx,
                                <li class="list-disc ml-10 text-lg text-blue"><a target="_blank" rel="noreferrer noopener" href={project.link}>{project.title}</a></li>
                                <ul>
                                    <li class="list-disc ml-20 text-md">{project.description}</li>
                                </ul>
                        }
                    }).collect::<Vec<_>>()
                }
                </ul>
            </div>
        </PageWrapper>
    }
}
