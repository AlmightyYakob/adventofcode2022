if [ -z "$1" ]
  then
    echo "Please supply a day as an integer"
    exit 1
fi

DAY=$1
DAYDIR="src/day$DAY"

if [ -d "$DAYDIR" ]; then
  echo "Directory for day $DAY already exists"
  exit 1
fi

cp -r src/day_template $DAYDIR
# wget -O "$DAYDIR/input.txt" "https://adventofcode.com/2022/day/$DAY"
touch "$DAYDIR/input.txt"
