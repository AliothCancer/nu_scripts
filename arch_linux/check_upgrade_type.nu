


def check_upgrade_type [old_version, new_version] {
    if $old_version.epoch != $new_version.epoch {
        return "epoch"
    } else if $old_version.major != $new_version.major {
        return "major"
    } else if $old_version.minor != $new_version.minor {
        return "minor"
    } else if $old_version.patch != $new_version.patch {
        return "patch"
    } else if $old_version.pkgrel != $new_version.pkgrel {
        return "pkgrel"
    } else {
        return "none"
    }
}
def get_updates [] {
    (checkupdates | lines | ansi strip | parse "{name} {old_version} -> {new_version}")
}
def create_json_file [json_file_name] {
    let packages = get_updates | select name
    let cache = {}
    for $pkg in packages {
        let link_variants = ["core", "extra", "community"]
        for variant in link_variants {
            let link = $"https://aur.archlinux.org/packages/($variant)/x86_64/$pkg/json"
            try {
                let pkgdesc = http get $link | get pkgdesc
                cache | insert $pkg $pkgdesc
                break
            } catch { |err| print $err.msg}
        }
    }
    $cache | save -f $json_file_name
}

def create_if_not_exists [json_file_name] {
    # will create a new json file if not exists returning true if the file has been created
    mut packages_description = null
    let exists = $json_file_name | path exists
    if not $exists {
        print "Creating json file for pkg description caching"
        create_json_file $json_file_name
        $packages_description = open $json_file_name | from json
        return true
    } else {
        print "File already exists"
        return false
    }
}
def get_not_cached_pkgdesc [json_cache, updates] {
    print "Getting not cached pakcages..."
    try {
        return $json_cache | select name | filter { |name| not ($name in $updates.name) }
    } catch { |err| print $err.msg; return {}}
}

let reg = '^(?:(?P<epoch>\d+):)?(?P<major>\d+)(?:\.(?P<minor>\d+))?(?:\.(?P<patch>\d+))?(?:-(?P<pkgrel>\d+))?';
let RED = $"(ansi red_bold)"
let RESET = $"(ansi reset)"
let release_order = ["epoch", "major", "minor", "patch", "pkgrel","none"]
def colorize_new_version [new_version: table, up_type: string, pkg_name: string] {

    #print $new_version
    let where_to_add_color: int = $release_order | enumerate | find $up_type | get index.0
    
    mut new_v_array: list<int> = [
        $new_version.epoch.0 ,
        $new_version.major.0,
        $new_version.minor.0,
        $new_version.patch.0,
        $new_version.pkgrel.0
    ];

    for $record in ($new_v_array | enumerate) {
        if ($record.item | is-empty) {
            $new_v_array = $new_v_array
                | drop nth $record.index
                | insert $record.index 0
        }
    }
    
    for $record in ($new_v_array | enumerate) {
        if ($record.index == $where_to_add_color) {
            $new_v_array = $new_v_array
                | drop nth $record.index
                | insert $record.index ($"(ansi green)" + $record.item + $"(ansi reset)")
            break
        }
    }
    

    return ( $new_v_array | str join ".")
    
}

def main [] {
    # contains the records pkg_name:desc
    #let json_file_name = "packages_desc.json"
    #let has_been_created = create_if_not_exists $json_file_name

    let updates = get_updates

    $updates | each { |row|

        let old_version = $row.old_version | parse --regex $reg
        let new_version = $row.new_version | parse --regex $reg
        let up_type = (check_upgrade_type $old_version $new_version)
        let j = {
            name: $row.name,
            update_type: $up_type,
            new_version: (colorize_new_version $new_version $up_type $row.name),
            #desc: ($json_cache | get $row.name),
        };
        $j
    }
    | sort-by { |x| ($release_order | enumerate | find $x.update_type) | get index.0 }
    | to csv
}
