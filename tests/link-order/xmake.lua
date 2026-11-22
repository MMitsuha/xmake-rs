target("alpha")
    set_kind("static")
    add_files("src/alpha.c")

target("beta")
    set_kind("static")
    add_files("src/beta.c")

target("gamma")
    set_kind("static")
    add_files("src/gamma.c")
    -- Exercise xmake-rs' vendored link-order sorting. The expected link
    -- order is the reverse of the default target order.
    add_linkorders("gamma", "beta", "alpha", {public = true})
