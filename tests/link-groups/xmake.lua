target("alpha")
    set_kind("static")
    add_files("src/alpha.c")

target("beta")
    set_kind("static")
    add_files("src/beta.c")

target("gamma")
    set_kind("static")
    add_files("src/gamma.c")
    -- Two linkgroups sharing the same name must be merged in declaration
    -- order (upstream xmake issue #5806).
    add_linkgroups("gamma", "beta", {name = "grp", public = true})
    add_linkgroups("alpha", {name = "grp", public = true})
