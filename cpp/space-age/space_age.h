#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

#include <stdint.h>

namespace space_age {
    
constexpr double oribital_period_earth_days = 365.25;
constexpr double oribital_period_earth_seconds = 31557600;
constexpr double oribital_period_mercury = 0.2408467;
constexpr double oribital_period_venus = 0.61519726;
constexpr double oribital_period_mars = 1.8808158;
constexpr double oribital_period_jupiter = 11.862615;
constexpr double oribital_period_saturn = 29.447498;
constexpr double oribital_period_uranus = 84.016846;
constexpr double oribital_period_neptune = 164.79132;

    class space_age{
    public:
        space_age(uint64_t age_seconds):_age(age_seconds){};
        int seconds() const {return _age;}
        double on_earth() const {return _age/oribital_period_earth_seconds;}
        double on_mercury() const {return on_earth()/oribital_period_mercury;}
        double on_venus() const {return on_earth()/oribital_period_venus;}
        double on_mars() const {return on_earth()/oribital_period_mars;}
        double on_jupiter() const {return on_earth()/oribital_period_jupiter;}
        double on_saturn() const {return on_earth()/oribital_period_saturn;}
        double on_uranus() const {return on_earth()/oribital_period_uranus;}
        double on_neptune() const {return on_earth()/oribital_period_neptune;}

    private:
        const uint64_t _age;
    };

}  // namespace space_age

#endif // SPACE_AGE_H