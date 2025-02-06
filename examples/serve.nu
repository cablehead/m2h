use xs.nu *

{|req|
  match $req {
    {method: "GET" , path: "/"} => {
      return (
        {
          content: (cat sample.md | m2h)
          theme: (m2h theme Dracula)
        } | to json -r | minijinja-cli -f json index.html -
      )
    }

    {method: "GET" , path: "/theme"} => {
      .response {headers: {"Content-Type": "text/event-stream"}}
      .cat -f | where topic == "theme" | each { .cas | lines | each { $"data: ($in)\n" } | append "\n" | str join }
    }

    _ => {
      .response {status: 404}
      return "sorry, eh"
    }
  }
}
