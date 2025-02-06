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
      .cat -f | each { to json -r | append "\n" | str join }
    }

    _ => {
      .response {status: 404}
      return "sorry, eh"
    }
  }
}
