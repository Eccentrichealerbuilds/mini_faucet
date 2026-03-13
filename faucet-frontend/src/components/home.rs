use crate::imports::*;

#[component]
pub fn Home() -> Element {
    rsx! { document::Stylesheet {href: FAUCET_CSS },
        div {class: "min-h-screen w-full bg-background flex items-center justify-center p-4 sm:p-6 md:p-8",
            div {class: "w-full max-w-lg flex flex-col gap-6",
                div { class: "bg-card border border-border rounded-lg shadow-lg
                    p-6 sm:p-8 flex flex-col items-center text-center space-y-4",
                    div {class: "h-14 w-14 bg-primary/10 text-primary
                        rounded-full flex items-center justify-center",
                        img { class:"h-8 w-8" ,src: asset!("/assets/droplet.svg") }
                    }
                    div { class: "space-y-1.5",
                        h1 { class: "text-2xl font-bold text-foreground tracking-tight",
                            "WELCOME TO SEPOLIA FAUCET"
                        }
                        p { class: "text-sm text-muted-foreground",
                            "Fast and reliable testnet ETH for developers." }
                    }
                }
                div {class: "bg-card border border-border rounded-lg 
                    shadow-lg overflow-hidden divide-y divide-border",
                    a{
                        Link { to: MyRoute::Claim,
                            class: "outline flex items-center gap-4 p-4 sm:px-6 transition-colors group 
                            bg-primary text-primary-foreground hover:bg-primary/90",
                            div {class: "h-10 w-10 rounded-full flex items-center justify-center 
                                shrink-0 bg-primary-foreground/15 border border-border",
                                img { class: "h-5 w-5",src: asset!("/assets/droplet.svg") }  
                            }
                            div { 
                                class: "flex-1 min-w-0",
                                p {class: "text-sm font-semibold", "Claim Page" }
                                p {class: "text-xs mt-0.5 text-primary-foreground/70", 
                                    "Request testnet ETH from the faucet"
                                }
                            }
                            img { src : asset!("/assets/chevron-right.svg"),
                                class: "h-4 w-4 shrink-0 transition-transform group-hover:translate-x-0.5 
                                text-muted-foreground",
                            }
                        }
                    }
                    a { 
                        Link { to: MyRoute::NextTime,
                            class: "flex items-center gap-4 p-4 sm:px-6 transition-colors group bg-card 
                            text-foreground hover:bg-accent hover:text-accent-foreground",
                            div{
                                class: "h-10 w-10 rounded-full flex items-center justify-center 
                                shrink-0 bg-muted border border-border ",
                                img{ class: "h-5 w-5", src: asset!("/assets/clock.svg")}
                            }
                            div {  
                                class: "flex-1 min-w-0",
                                p { class: "text-sm font-semibold", "Check Next Claim Time" }
                                p { class: "text-xs mt-0.5 text-muted-foreground", "See when you can claim again" }
                            }
                            img { class: "h-4 w-4 shrink-0 transition-transform group-hover:translate-x-0.5 
                                text-muted-foreground",
                                src : asset!("/assets/chevron-right.svg"),
                            }
                        }
                    }
                    a { 
                        Link{class: "outline flex items-center gap-4 p-4 sm:px-6 transition-colors group bg-card 
                            text-foreground hover:bg-accent hover:text-accent-foreground",
                            to: MyRoute::MyBalance,
                            div{
                                class: "h-10 w-10 rounded-full flex items-center justify-center 
                                shrink-0 bg-muted border border-border bg-sky-500",
                                img {class: "h-5 w-5", src: asset!("/assets/wallet-minimal.svg")}
                            }
                            div { 
                                class: "flex-1 min-w-0",
                                p { class: "text-sm font-semibold", "Check Balance" }
                                p { class: "text-xs mt-0.5 text-muted-foreground", "View your current sepolia balance" }
                            }
                            img { class: "h-4 w-4 shrink-0 transition-transform group-hover:translate-x-0.5 
                                text-muted-foreground",
                                src : asset!("/assets/chevron-right.svg"),
                            }
                        }
                    }
                    a { 
                        Link{ class: "flex items-center gap-4 p-4 sm:px-6 transition-colors group bg-card
                            text-foreground hover:bg-accent hover:text-accent-foreground",
                            to: MyRoute::FaucetBalance,
                            div{
                                class: "h-10 w-10 rounded-full flex items-center justify-center shrink-0 
                                bg-muted border border-border",
                                img { 
                                    class: "h-5 w-5", src: asset!("/assets/wallet.svg")
                                 }
                            }
                            div{
                                class: "flex-1 min-w-0",
                                p { class: "text-sm font-semibold", "Faucet Balance" }
                                p{
                                    class: "text-xs mt-0.5 text-muted-foreground", "Check the faucet available funds"
                                }
                            }
                            img { class: "h-4 w-4 shrink-0 transition-transform group-hover:translate-x-0.5 
                                text-muted-foreground",
                                src : asset!("/assets/chevron-right.svg"),
                            }
                        }
                    }
                    a{
                        Link{
                            class: "outline flex items-center gap-4 p-4 sm:px-6 transition-colors group bg-card 
                            text-foreground hover:bg-accent hover:text-accent-foreground",
                            to: MyRoute::Contribute,
                            div{
                                class:"h-10 w-10 rounded-full flex items-center justify-center 
                                shrink-0 bg-muted border border-border",
                                img{
                                    class:"h-5 h-5", src: asset!("/assets/hand-heart.svg")
                                }
                            }
                            div {
                                class:"flex-1 min-w-0",
                                p { class: "text-sm font-semibold", "Contribute" }
                                p{class: "text-xs mt-0.5 text-muted-foreground", 
                                "Donate sepolia back to the faucet"}
                            }
                            img { class: "h-4 w-4 shrink-0 transition-transform group-hover:translate-x-0.5 
                                text-muted-foreground",
                                src : asset!("/assets/chevron-right.svg"),
                            }
                        }
                    }                    
                }
                div {  
                    class: "text-center items-center",
                    p {  
                        class: "text-xs text-muted-foreground",
                        "Developer: Eccentric Healer"
                    }
                    a{href: "https://x.com/eccentrichealer",
                        img {class:"h-7 w-7 mx-auto",
                        src: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAYAAABXAvmHAAAACXBIWXMAAAsTAAALEwEAmpwYAAACeklEQVR4nO2Zu2tUQRjFf8YIvhDEB4IWGqwUG41GNIKNBmLlq/b5F1gpInb+AS6rYK0mNsZCLCxEFG0UQTBRsBHXVaw0UQSTjSMD58KwbNyd2b07E7wHTnFn7sz9DjPnft/cCwUKFCgwH7APGAWqQA0wObEGfAJGgMFOBN4LXM8xYNOEZcUQjJjBG0dE8LYxiXAwRMBoAoEb8VaIgGoCgRvRGtsbeb5tfFkLEWASozf+NdkMsGuOcadbDOg+sBA41OJqd1SA5QSwBFgAHAaOAzs09m6TsS+AZbr/Z4wVyFjSvZd1/R3YCKwGPs8xpgKsFyuxtlDGP8CwsuVztT3V1jiofvf+SWAbsAJ4HdMDLr8Ca4E+YEpt5zVH2blvGjgALAIexjaxqeOYxpx1gt0JLAbeaCVOtlmeeMP3ASc07p6ux2XyfuCS+i4EBm/8w/d/wA9gswz8RW1XnfnsW2o2ZQEGeCIDD2vbWA5pvkdtBG+6JaCRgTOTb3JMnrSAaRl4KfC2zuRn5oOAX8CA5uiXINt+Sm1jKQuYBY6ovDiqeS6qb0q5Yo1jcpOagHMae8XJ0j3AY/U/k8mHGmRpE1vAjQbVqD0UrZKBJ9VmV8TiWkoCHqge2g/8ruuzlSnyQFaKD8jk71IQ8ApYDmwBvjXJ0nd0/V5jtjsmjyLAbpENwDrgQwtZeiXwUW12C6EyI4qArDS2B5OXHll6q8qKY6pM3VK8awJmVO/3eL7Xsyxdj74WsnTHV6Db/P8+q1QTCNyI9uzsjZEEAjfizRABuwPSfV7cSyDcQ3kslmgDvZFFlNr9wZHBLuFtmSnvX0wVfU7f04nACxQoUIBc8RdPHHCGhTScAwAAAABJRU5ErkJggg==" }
                        
                    }

                }
            }
        }
    }
}
