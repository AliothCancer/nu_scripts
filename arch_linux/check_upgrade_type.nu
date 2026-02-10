#!/usr/bin/nu


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


let reg = '^(?:(?P<epoch>\d+):)?(?P<major>\d+)(?:\.(?P<minor>\d+))?(?:\.(?P<patch>\d+))?(?:-(?P<pkgrel>\d+))?';
let RED = $"(ansi red_bold)"
let GREEN = $"(ansi green_bold)"
let RESET = $"(ansi reset)"

let release_order = ["epoch", "major", "minor", "patch", "pkgrel","none"]

# age can be "new"|"old"
def colorize_version_string [version: table, up_type: string, age: string] {
    let where_to_add_color: int = $release_order | enumerate | find $up_type | get index.0
    mut color: string = "";
    match $age {
        "new" => ($color = $GREEN)
        "old" => ($color = $RED)
        _ => (error make "age must be old or new")
    }

    # If version is only empty values, they occupy the place anyway
    # so the list is never empty even if all values are null
    mut new_v_array: list<oneof<nothing, int>> = [
        $version.epoch.0 , # these values are all possibly null
        $version.major.0,
        $version.minor.0,
        $version.patch.0,
        $version.pkgrel.0
    ];
    let to_colorize: oneof<nothing, string>  = $new_v_array | get --optional $where_to_add_color
    
    $new_v_array = $new_v_array | drop nth $where_to_add_color
    
    $new_v_array = $new_v_array | insert $where_to_add_color ( $color + ($to_colorize | default "0") + $RESET )
    
    
    let ver_str_tail = ($new_v_array | last)
    mut new_ver_str = ( $new_v_array | take 4 | where {|x| $x| $x != null} | str join ".")

    if $ver_str_tail != null {
        $new_ver_str += "-" + $ver_str_tail
    }
    
    return $new_ver_str
    
}

def main [] {

    let updates = get_updates

    $updates | each { |row|

        let old_version = $row.old_version | parse --regex $reg
        let new_version = $row.new_version | parse --regex $reg
        let up_type = (check_upgrade_type $old_version $new_version)
        let j = {
            name: $row.name,
            update_type: $up_type,
            new_version: (colorize_version_string $new_version $up_type "new"),
            #new_version_orig: $row.new_version,
            old_version: (colorize_version_string $old_version $up_type "old")
        }
        $j
    }
    | sort-by { |x| ($release_order | enumerate | find $x.update_type) | get index.0 }
    | to csv
}
